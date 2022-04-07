use crate::bathymetry::{Bathymetry, Point};

use std::cmp::Reverse;
use std::collections::HashMap;

use geo::algorithm::line_interpolate_point::LineInterpolatePoint;
use geo::Line;

use priority_queue::PriorityQueue;

use rstar::RTree;

pub struct ThalwegGenerator {
    points: RTree<Bathymetry>,
    resolution: f64,
}

impl ThalwegGenerator {
    pub fn from_points(points: Vec<Bathymetry>, resolution: usize) -> Self {
        Self {
            points: RTree::bulk_load(points),
            resolution: resolution as f64,
        }
    }

    pub fn thalweg(&self, source: Point, sink: Point) -> Option<Vec<Bathymetry>> {
        let source_in_tree = self.points.nearest_neighbor(&source)?;
        let sink_in_tree = self.points.nearest_neighbor(&sink)?;

        // RTree uses distance^2 in locate_within_distance
        let distance_squared = self.resolution * self.resolution;

        let mut state = HashMap::new();
        state.insert(source_in_tree, (0.0, source_in_tree));
        let mut work_queue = PriorityQueue::new();
        work_queue.push(source_in_tree, Reverse(0));

        while let Some((current, _)) = work_queue.pop() {
            let distance_to_here = state.get(current).map(|&(d, _)| d).unwrap_or(f64::INFINITY);

            for neighbor in self
                .points
                .locate_within_distance(current.point(), distance_squared)
            {
                // use A* names to make comparison easier
                let g_n = distance_to_here + 1.0;
                let old_distance = state
                    .get(&neighbor)
                    .map(|&(d, _)| d)
                    .unwrap_or(f64::INFINITY);
                if g_n < old_distance {
                    state.insert(neighbor, (g_n, current));
                    let h_n = neighbor.distance_to(sink_in_tree);
                    let f_n = g_n + h_n;
                    // push_increase will do the insertion as normal if the neighbor is not already present,
                    // but will modify the priority if we hand it a "larger" one. Most descriptions of A* use a
                    // min heap, where decrease_priority works as expected. However, work_queue is implemented
                    // as a max heap, which we are tricking into becoming a min heap with Reverse(). Because of
                    // this, increased priorities are actually smaller numbers, as desired.
                    work_queue.push_increase(neighbor, Reverse(f_n as isize));
                }
            }

            if current == sink_in_tree {
                break;
            }
        }

        if !state.contains_key(sink_in_tree) {
            return None;
        }

        let mut path = vec![];
        let mut current = sink_in_tree;
        while current != source_in_tree {
            path.push(current.clone());
            current = self
                .points
                .nearest_neighbor(&state.get(&current).map(|&(_, p)| p.clone())?.point())?;
        }
        path.push(current.clone());

        path.reverse();
        Some(path)
    }

    pub fn sink(&self, points: &[Bathymetry]) -> Vec<Bathymetry> {
        let mut out = vec![];

        if let Some(point) = points.first() {
            out.push(point.clone());
        }

        for window in points.windows(3) {
            let prev = &window[0];
            let current = &window[1];
            let next = &window[2];
            let dist_1 = prev.distance_to(current);
            let dist_2 = current.distance_to(next);
            let dist = f64::min(dist_1, dist_2);
            // avoid overlapping with neighbors
            let resolution = dist / 2.0;
            // RTree uses distance^2 in locate_within_distance
            let distance_squared = resolution * resolution;

            let best_neighbor = self
                .points
                .locate_within_distance(current.point(), distance_squared)
                .fold(current, |best, neighbor| {
                    if best.depth() < neighbor.depth() {
                        neighbor
                    } else {
                        best
                    }
                });
            out.push(best_neighbor.clone());
        }

        if let Some(point) = points.last() {
            out.push(point.clone());
        }

        out
    }

    pub fn shrink(&self, points: &[Bathymetry]) -> Vec<Bathymetry> {
        let mut out = vec![];

        let factor = self.resolution / 2.0;

        if let Some(point) = points.first() {
            out.push(point.clone());
        }

        for window in points.windows(2) {
            let prev = &window[0];
            let current = &window[1];
            if prev.distance_to(current) < factor && prev.depth() > current.depth() {
                continue;
            }
            out.push(current.clone());
        }

        out
    }

    pub fn with_midpoints(&self, points: &[Bathymetry]) -> Vec<Bathymetry> {
        let mut out = vec![];

        for window in points.windows(2) {
            let a = &window[0];
            let b = &window[1];

            out.push(a.clone());
            let line = Line::new(a.point(), b.point());
            if let Some(midpoint) = line
                .line_interpolate_point(0.5)
                .and_then(|m| self.points.nearest_neighbor(&m.x_y()))
            {
                out.push(midpoint.clone());
            }
            out.push(b.clone());
        }

        // added numerous points twice - get rid of them
        out.dedup();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thalweg_provides_nodes_containing_source_and_sink() {
        let data = vec![
            Bathymetry::new(0.0, 0.0, 0.0),
            Bathymetry::new(0.00001, 0.00001, 0.0),
        ];
        let generator = ThalwegGenerator::from_points(data.clone(), 400);
        let path = generator.thalweg(data.first().unwrap().point(), data.last().unwrap().point());
        assert_eq!(path, Some(data));
    }

    #[test]
    fn thalweg_ignores_nodes_not_between_source_and_sink() {
        let data = vec![
            Bathymetry::new(0.0, 0.0, 0.0),
            Bathymetry::new(0.00001, 0.00001, 0.0),
            Bathymetry::new(-1.0, -1.0, 0.0),
        ];
        let expected = vec![
            Bathymetry::new(0.0, 0.0, 0.0),
            Bathymetry::new(0.00001, 0.00001, 0.0),
        ];
        let generator = ThalwegGenerator::from_points(data, 400);
        let path = generator.thalweg(
            expected.first().unwrap().point(),
            expected.last().unwrap().point(),
        );
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn thalweg_provides_a_path_ignoring_depth() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let data = vec![
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, 0.0 * one_second, 150.0 * km),
            Bathymetry::new(-1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(0.0 * one_second, 1.0 * one_second, 140.0 * km),
            Bathymetry::new(1.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
        ];
        let expected = vec![data[0].clone(), data[4].clone(), data[8].clone()];
        let generator = ThalwegGenerator::from_points(data, 50);
        let path = generator.thalweg(
            expected.first().unwrap().point(),
            expected.last().unwrap().point(),
        );
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn sink_provides_a_path_with_the_same_number_of_points() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let data = vec![
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, 0.0 * one_second, 150.0 * km),
            Bathymetry::new(-1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(0.0 * one_second, 1.0 * one_second, 140.0 * km),
            Bathymetry::new(1.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
        ];
        let input = vec![data[0].clone(), data[4].clone(), data[8].clone()];
        let generator = ThalwegGenerator::from_points(data, 50);
        let path = generator.sink(&input);
        assert_eq!(path.len(), input.len());
    }

    #[test]
    fn sink_provides_a_deeper_path() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let data = vec![
            Bathymetry::new(-2.0 * one_second, -2.0 * one_second, 10.0 * km),
            Bathymetry::new(-2.0 * one_second, -1.0 * one_second, 200.0 * km),
            Bathymetry::new(-2.0 * one_second, 0.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, -2.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, 0.0 * one_second, 150.0 * km),
            Bathymetry::new(-1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(0.0 * one_second, 1.0 * one_second, 140.0 * km),
            Bathymetry::new(0.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, 2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(2.0 * one_second, 2.0 * one_second, 5.0 * km),
        ];
        let input = vec![data[0].clone(), data[12].clone(), data[24].clone()];
        let expected = vec![data[0].clone(), data[7].clone(), data[24].clone()];
        let generator = ThalwegGenerator::from_points(data, 50);
        let path = generator.sink(&input);
        assert_eq!(path, expected);
    }

    #[test]
    fn shrink_does_not_modify_values() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let input = vec![
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
        ];
        let generator = ThalwegGenerator::from_points(vec![], 40);
        let new_path = generator.shrink(&input);
        for point in new_path {
            assert!(input.iter().any(|elem| *elem == point));
        }
    }

    #[test]
    fn shrink_can_shrink_path() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let input = vec![
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
        ];
        let generator = ThalwegGenerator::from_points(vec![], 100);
        let new_path = generator.shrink(&input);
        assert!(new_path.len() < input.len());
    }

    #[test]
    fn with_midpoint_produces_new_values_between_existing_ones() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let data = vec![
            Bathymetry::new(-2.0 * one_second, -2.0 * one_second, 10.0 * km),
            Bathymetry::new(-2.0 * one_second, -1.0 * one_second, 200.0 * km),
            Bathymetry::new(-2.0 * one_second, 0.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, -2.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, 0.0 * one_second, 150.0 * km),
            Bathymetry::new(-1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(0.0 * one_second, 1.0 * one_second, 140.0 * km),
            Bathymetry::new(0.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, 2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(2.0 * one_second, 2.0 * one_second, 5.0 * km),
        ];
        let input = vec![data[0].clone(), data[24].clone()];
        let expected = vec![data[0].clone(), data[12].clone(), data[24].clone()];
        let generator = ThalwegGenerator::from_points(data, 50);
        let path = generator.with_midpoints(&input);
        assert_eq!(path, expected);
    }

    #[test]
    fn with_midpoint_applied_twice_subdivides_further() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let data = vec![
            Bathymetry::new(-2.0 * one_second, -2.0 * one_second, 10.0 * km),
            Bathymetry::new(-2.0 * one_second, -1.0 * one_second, 200.0 * km),
            Bathymetry::new(-2.0 * one_second, 0.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, -2.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, 0.0 * one_second, 150.0 * km),
            Bathymetry::new(-1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(0.0 * one_second, 1.0 * one_second, 140.0 * km),
            Bathymetry::new(0.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, 2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(2.0 * one_second, 2.0 * one_second, 5.0 * km),
        ];
        let input = vec![data[0].clone(), data[24].clone()];
        let expected = vec![
            data[0].clone(),
            data[6].clone(),
            data[12].clone(),
            data[18].clone(),
            data[24].clone(),
        ];
        let generator = ThalwegGenerator::from_points(data, 50);
        let path = generator.with_midpoints(&generator.with_midpoints(&input));
        assert_eq!(path, expected);
    }

    #[test]
    fn with_midpoint_does_not_invent_data() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let data = vec![
            Bathymetry::new(-2.0 * one_second, -2.0 * one_second, 10.0 * km),
            Bathymetry::new(-2.0 * one_second, -1.0 * one_second, 200.0 * km),
            Bathymetry::new(-2.0 * one_second, 0.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-2.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, -2.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, 0.0 * one_second, 150.0 * km),
            Bathymetry::new(-1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(-1.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -2.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, -1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0 * one_second, 0.0 * one_second, 9.0 * km),
            Bathymetry::new(0.0 * one_second, 1.0 * one_second, 140.0 * km),
            Bathymetry::new(0.0 * one_second, 2.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(1.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(1.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(1.0 * one_second, 2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -2.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(2.0 * one_second, 0.0 * one_second, 6.0 * km),
            Bathymetry::new(2.0 * one_second, 1.0 * one_second, 100.0 * km),
            Bathymetry::new(2.0 * one_second, 2.0 * one_second, 5.0 * km),
        ];
        let input = vec![
            data[0].clone(),
            data[6].clone(),
            data[12].clone(),
            data[18].clone(),
            data[24].clone(),
        ];
        let generator = ThalwegGenerator::from_points(data, 50);
        let path = generator.with_midpoints(&generator.with_midpoints(&input));
        assert_eq!(path, input);
    }
}
