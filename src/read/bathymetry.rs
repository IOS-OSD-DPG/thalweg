use crate::bathymetry::Bathymetry;
use crate::parse;

use std::error::Error;
use std::io::{BufRead, BufReader, Read};

/// Read bathymetry data from the NONNA-10 ASCII format
pub fn from_nonna<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    let mut out = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break Ok(out);
        }
        if let Some(value) = nonna_line(&buffer) {
            out.push(value);
        }
    }
}

fn nonna_line(input: &str) -> Option<Bathymetry> {
    let mut split = input.split(' ');
    let latitude = parse::parse_dms_latitude(split.next()?)?;
    let longitude = parse::parse_dms_longitude(split.next()?)?;
    let depth = parse::parse_float(split.next()?)?;
    Some(Bathymetry::new(latitude, longitude, depth))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_bathymetry_lines() {
        let source = "0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader);
        let expected = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_multiple_bathymetry_lines() {
        let source = "0-0-0.0N 0-0-0.0E 0.0\n0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader);
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
        let actual = from_nonna(&mut reader);
        let expected = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
