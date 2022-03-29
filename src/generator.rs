use crate::bathymetry::{Bathymetry, Point};

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

use rstar::RTree;

pub struct ThalwegGenerator {
    points: RTree<Bathymetry>,
    max_depth: f64,
    resolution: usize,
}

impl ThalwegGenerator {
    pub fn from_points(points: Vec<Bathymetry>, resolution: usize) -> Self {
        let max_depth = points
            .iter()
            .map(Bathymetry::depth)
            .reduce(f64::max)
            .expect("no points given to ThalwegGenerator");
        Self {
            points: RTree::bulk_load(points),
            max_depth,
            resolution,
        }
    }

    pub fn thalweg(&self, source: Point, sink: Point) -> Option<Vec<Bathymetry>> {
        let source_in_tree = self.points.nearest_neighbor(&source)?;
        let sink_in_tree = self.points.nearest_neighbor(&sink)?;

        // RTree uses distance^2 in locate_within_distance
        let distance_squared = (self.resolution * self.resolution) as f64;

        let mut visited = HashSet::new();
        let mut state = HashMap::new();
        let mut work_queue = PriorityQueue::new();
        work_queue.push(source_in_tree, Reverse(0));

        while let Some((current, _)) = work_queue.pop() {
            let distance_to_here = state.get(current).map(|&(d, _)| d).unwrap_or(f64::INFINITY);

            for neighbor in self
                .points
                .locate_within_distance(current.point(), distance_squared)
            {
                // use A* names for to make comparison easier
                let g_n = distance_to_here + self.weight_of(neighbor);
                let old_distance = state
                    .get(&neighbor)
                    .map(|&(d, _)| d)
                    .unwrap_or(f64::INFINITY);
                if !state.contains_key(&neighbor) || g_n < old_distance {
                    state.insert(neighbor, (g_n, current));
                    let h_n = neighbor.distance_to(&sink_in_tree);
                    let f_n = g_n + h_n;
                    work_queue.push(neighbor, Reverse(f_n as isize));
                }
            }
            visited.insert(current.location());

            if visited.contains(&sink_in_tree.location()) {
                break;
            }
        }

        if !visited.contains(&sink_in_tree.location()) {
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

    fn weight_of(&self, point: &Bathymetry) -> f64 {
        self.max_depth - point.depth() + 1.0
    }
}

#[cfg(test)]
mod test {
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
        let generator = ThalwegGenerator::from_points(data.clone(), 400);
        let path = generator.thalweg(
            expected.first().unwrap().point(),
            expected.last().unwrap().point(),
        );
        assert_eq!(path, Some(expected));
    }

    #[test]
    fn thalweg_provides_a_path() {
        let km = 1000.0;
        let one_second = 1.0 / 3600.0;
        let data = vec![
            Bathymetry::new(-1.0 * one_second, -1.0 * one_second, 140.0 * km),
            Bathymetry::new(-1.0 * one_second, 0.0, 150.0 * km),
            Bathymetry::new(-1.0 * one_second, one_second, 100.0 * km),
            Bathymetry::new(0.0, -1.0 * one_second, 100.0 * km),
            Bathymetry::new(0.0, 0.0, 9.0 * km),
            Bathymetry::new(0.0, one_second, 140.0 * km),
            Bathymetry::new(one_second, -1.0 * one_second, 5.0 * km),
            Bathymetry::new(one_second, 0.0, 6.0 * km),
            Bathymetry::new(one_second, one_second, 100.0 * km),
        ];
        let expected = vec![
            data[0].clone(),
            data[1].clone(),
            data[4].clone(),
            data[7].clone(),
            data[8].clone(),
        ];
        let generator = ThalwegGenerator::from_points(data, 40);
        let path = generator.thalweg(
            expected.first().unwrap().point(),
            expected.last().unwrap().point(),
        );
        assert_eq!(path, Some(expected));
    }
}
