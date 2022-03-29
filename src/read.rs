use crate::bathymetry::{Bathymetry, Point};
use crate::parse;

use std::io::{self, BufRead, BufReader, Read};

pub fn read_data_lines<T: Read>(input: &mut BufReader<T>) -> io::Result<Vec<Bathymetry>> {
    let mut out = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        let bytes_read = input.read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        if let Some(value) = read_data_line(buffer.trim()) {
            out.push(value);
        }
    }
    Ok(out)
}

pub fn read_corner_lines<T: Read>(input: &mut BufReader<T>) -> io::Result<Vec<Point>> {
    let mut out = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        let bytes_read = input.read_line(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        if let Some(value) = read_corner_line(buffer.trim()) {
            out.push(value);
        }
    }
    Ok(out)
}

fn read_data_line(input: &str) -> Option<Bathymetry> {
    let mut split = input.split(' ');
    let latitude = parse::parse_dms_latitude(split.next()?)?;
    let longitude = parse::parse_dms_longitude(split.next()?)?;
    let depth = parse::parse_float(split.next()?)?;
    Some(Bathymetry::new(latitude, longitude, depth))
}

fn read_corner_line(input: &str) -> Option<Point> {
    let mut split = input.split(' ');
    let latitude = parse::parse_dms_latitude(split.next()?)?;
    let longitude = parse::parse_dms_longitude(split.next()?)?;
    Some((longitude, latitude))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reads_bathymetry_lines() {
        let source = "0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_data_lines(&mut reader);
        let expected = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_multiple_bathymetry_lines() {
        let source = "0-0-0.0N 0-0-0.0E 0.0\n0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_data_lines(&mut reader);
        let expected = vec![
            Bathymetry::new(0.0, 0.0, 0.0),
            Bathymetry::new(0.0, 0.0, 0.0),
        ];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn ignores_non_bathymetry_lines() {
        let source = "not actual bathymetry\n0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_data_lines(&mut reader);
        let expected = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_corner_lines() {
        let source = "0-0-0.0N 0-0-0.0E";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_lines(&mut reader);
        let expected = vec![(0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_multiple_corner_lines() {
        let source = "0-0-0.0N 0-0-0.0E\n0-0-0.0N 0-0-0.0E";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_lines(&mut reader);
        let expected = vec![(0.0, 0.0), (0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn rejects_non_corner_lines() {
        let source = "not actual corner\n0-0-0.0N 0-0-0.0E";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_lines(&mut reader);
        let expected = vec![(0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
