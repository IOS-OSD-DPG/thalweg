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
    let potential = input
        .chars()
        .skip_while(|c| !in_float(c))
        .take_while(in_float)
        .collect::<String>();
    if let Ok(value) = potential.parse::<f64>() {
        Some(value)
    } else {
        None
    }
}

fn trim_last(input: &str) -> &str {
    &input[..input.len()-1]
}

fn parse_dms_coord(input: &str, bound: usize, factor: f64) -> Option<f64> {
    let sections: Vec<&str> = input.split('-').collect();
    if sections.len() != 3 {
        return None;
    }
    let degrees = sections[0].parse::<usize>().ok()?;
    if degrees > bound {
        return None;
    }
    let minutes = sections[1].parse::<usize>().ok()?;
    if minutes > 59 {
        return None;
    }
    let seconds = sections[2].parse::<f64>().ok()?;
    if seconds < 0.0 || seconds >= 60.0 {
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

fn in_float(&c: &char) -> bool {
    c == '-' || c == '.' || c.is_digit(10) || c == 'e' || c == 'E'
}

#[cfg(test)]
mod test {
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
}
