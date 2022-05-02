use geo::{LineString, Polygon};

pub fn parse_dms_latitude(latitude: &str) -> Option<f64> {
    let trimmed_lat = trim_last(latitude);
    if latitude.ends_with(is_north) {
        parse_dms_coord(trimmed_lat, 90, 1.0)
    } else if latitude.ends_with(is_south) {
        parse_dms_coord(trimmed_lat, 90, -1.0)
    } else {
        None
    }
}

pub fn parse_dms_longitude(longitude: &str) -> Option<f64> {
    let trimmed_lon = trim_last(longitude);
    if longitude.ends_with(is_east) {
        parse_dms_coord(trimmed_lon, 180, 1.0)
    } else if longitude.ends_with(is_west) {
        parse_dms_coord(trimmed_lon, 180, -1.0)
    } else {
        None
    }
}

pub fn parse_float(input: &str) -> Option<f64> {
    input.trim().parse::<f64>().ok()
}

pub fn parse_bounding_box(input: &str) -> Option<Polygon<f64>> {
    let coll: Vec<Option<f64>> = input.split(',')
         .take(4)
         .map(parse_float)
         .collect();
    if coll.len() < 4 || coll.iter().any(|o| o.is_none()) {
        None
    } else {
        let output = Polygon::new(
            LineString::from(vec![
                (coll[0].unwrap(), coll[1].unwrap()),
                (coll[2].unwrap(), coll[1].unwrap()),
                (coll[2].unwrap(), coll[3].unwrap()),
                (coll[0].unwrap(), coll[3].unwrap()),
                (coll[0].unwrap(), coll[1].unwrap())
            ]),
            vec![],
        );
        Some(output)
    }
}

fn trim_last(input: &str) -> &str {
    &input[..input.len() - 1]
}

fn parse_dms_coord(input: &str, bound: usize, factor: f64) -> Option<f64> {
    let sections: Vec<&str> = input.split('-').collect();
    if sections.len() != 3 {
        return None;
    }
    let degrees = sections[0].parse::<usize>().ok()?;
    if !(0..bound).contains(&degrees) {
        return None;
    }
    let minutes = sections[1].parse::<usize>().ok()?;
    if !(0..60).contains(&minutes) {
        return None;
    }
    let seconds = sections[2].parse::<f64>().ok()?;
    if !(0.0..60.0).contains(&seconds) {
        return None;
    }
    Some(factor * (degrees as f64 + (minutes as f64 / 60.0) + (seconds / 3600.0)))
}

fn is_north(c: char) -> bool {
    c == 'n' || c == 'N'
}

fn is_south(c: char) -> bool {
    c == 's' || c == 'S'
}

fn is_east(c: char) -> bool {
    c == 'e' || c == 'E'
}

fn is_west(c: char) -> bool {
    c == 'w' || c == 'W'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_proper_latitude() {
        assert_eq!(parse_dms_latitude("49-12-00.000N"), Some(49.2));
        assert_eq!(parse_dms_latitude("49-12-00.000S"), Some(-49.2));
    }

    #[test]
    fn rejects_improper_latitude() {
        assert_eq!(parse_dms_latitude("not a latitude"), None);
    }

    #[test]
    fn reads_proper_longitude() {
        assert_eq!(parse_dms_longitude("122-56-24.360E"), Some(122.9401));
        assert_eq!(parse_dms_longitude("122-56-24.360W"), Some(-122.9401));
    }

    #[test]
    fn rejects_improper_longitude() {
        assert_eq!(parse_dms_longitude("not a longitude"), None);
    }

    #[test]
    fn reads_proper_float() {
        assert_eq!(parse_float("-0.99565"), Some(-0.99565));
    }

    #[test]
    fn rejects_improper_float() {
        assert_eq!(parse_float("not a float"), None);
    }

    #[test]
    fn reads_bounding_box() {
        let outline = vec![
            (-123.4, 54.3),
            (-123.9, 54.3),
            (-123.9, 55.0),
            (-123.4, 55.0),
            (-123.4, 54.3),
        ];
        let expected = Polygon::new(
            LineString::from(outline),
            vec![]
        );
        assert_eq!(parse_bounding_box("-123.4,54.3,-123.9,55.0"), Some(expected));
    }

    #[test]
    fn rejects_invalid_box() {
        assert_eq!(parse_bounding_box("not a bounding box"), None);
        assert_eq!(parse_bounding_box("-123.4"), None);
        assert_eq!(parse_bounding_box("-123.4,missing,-123.9,55.0"), None);
    }
}
