use crate::bathymetry::Bathymetry;

#[derive(Debug, PartialEq)]
pub struct Section {
    distance: isize,
    depth: f64,
}

impl Section {
    fn new(distance: isize, depth: f64) -> Self {
        Self { distance, depth }
    }
}

pub fn section(path: Vec<Bathymetry>) -> Vec<Section> {
    let mut distance_from_start = 0;
    let mut out = vec![];
    if let Some(start) = path.get(0) {
        out.push(Section::new(distance_from_start, start.depth()));
    }
    for window in path.windows(2) {
        let start = window[0].clone();
        let end = window[1].clone();
        distance_from_start += start.distance_to(&end) as isize;
        out.push(Section::new(distance_from_start, end.depth()));
    }
    out
}

pub fn to_csv(section: Vec<Section>) -> String {
    let mut out = String::from("distance,depth\n");
    for elem in section {
        out += format!("{},{}\n", elem.distance as f64 / 1000.0, elem.depth).as_str();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_from_single_point() {
        let input = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert_eq!(section(input), vec![Section::new(0, 0.0)]);
    }

    #[test]
    fn section_from_many_points() {
        let input = vec![
            Bathymetry::new(49.24, -122.59, 157.692),
            Bathymetry::new(49.24, -122.53, 96.996),
            Bathymetry::new(49.24, -122.46, 107.072),
        ];
        let expected = vec![
            Section::new(0, 157.692),
            Section::new(4355, 96.996),
            Section::new(9436, 107.072),
        ];
        assert_eq!(section(input), expected);
    }

    #[test]
    fn section_to_csv_no_points() {
        let input = vec![];
        let expected = "distance,depth\n";
        assert_eq!(to_csv(input), expected);
    }

    #[test]
    fn section_to_csv_one_point() {
        let input = vec![Section::new(0, 157.692)];
        let expected = "distance,depth\n0,157.692\n";
        assert_eq!(to_csv(input), expected);
    }

    #[test]
    fn section_to_csv_many_points() {
        let input = vec![Section::new(0, 157.692), Section::new(4355, 96.996)];
        let expected = "distance,depth\n0,157.692\n4.355,96.996\n";
        assert_eq!(to_csv(input), expected);
    }
}
