use crate::bathymetry::{Bathymetry, Point};
use crate::parse;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

pub fn read_data_lines<T: Read>(
    input: &mut BufReader<T>,
) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    let mut out = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        if let Some(value) = read_data_line(buffer.trim()) {
            out.push(value);
        }
    }
    Ok(out)
}

pub fn read_corner_lines<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut out = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        if let Some(value) = read_corner_line(buffer.trim()) {
            out.push(value);
        }
    }
    Ok(out)
}

pub fn read_corner_csv<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut out = vec![];
    let mut buffer = String::new();
    // read header
    if input.read_line(&mut buffer)? == 0 {
        return Ok(out);
    }
    let mut column_map = HashMap::new();
    for (index, item) in buffer.split(',').enumerate() {
        column_map.insert(item.trim().trim_matches('"'), index);
    }
    let latitude_index = column_map
        .keys()
        .filter(|name| name.to_lowercase().starts_with("la"))
        .next()
        .and_then(|key| column_map.get(key));
    let longitude_index = column_map
        .keys()
        .filter(|name| name.to_lowercase().starts_with("lo"))
        .next()
        .and_then(|key| column_map.get(key));
    if latitude_index.is_none() || longitude_index.is_none() {
        return Ok(out);
    }
    let &latitude_index = latitude_index.unwrap();
    let &longitude_index = longitude_index.unwrap();
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break;
        }
        let row: Vec<&str> = buffer.split(',').collect();
        let latitude = parse::parse_float(row[latitude_index]);
        let longitude = parse::parse_float(row[longitude_index]);
        if latitude.is_none() || longitude.is_none() {
            continue;
        }
        out.push((longitude.unwrap(), latitude.unwrap()));
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
mod tests {
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

    #[test]
    fn reads_corner_csv_longitude_first() {
        let source = "longitude,latitude\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_corner_csv_latitude_first() {
        let source = "latitude,longitude\n49.58,-123.456";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_corner_csv_short_names() {
        let source = "lon,lat\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_corner_csv_capitalized_names() {
        let source = "Longitude,Latitiude\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_corner_csv_despite_formatting() {
        let source = "\"longitude (float)\", \"latitiude (float)\"\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = read_corner_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
