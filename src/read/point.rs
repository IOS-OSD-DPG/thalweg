use crate::bathymetry::Point;
use crate::parse;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

/// Read point data from the NONNA-10 ASCII format
pub fn from_nonna<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut out = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break Ok(out);
        }
        if let Some(value) = nonna_line(buffer.trim()) {
            out.push(value);
        }
    }
}

/// Read point data from a CSV
pub fn from_csv<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Point>, Box<dyn Error>> {
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
    let latitude_index = *column_map
        .keys()
        .find(|name| name.to_lowercase().starts_with("la"))
        .and_then(|key| column_map.get(key))
        .ok_or("Latitude not found")?;
    let longitude_index = *column_map
        .keys()
        .find(|name| name.to_lowercase().starts_with("lo"))
        .and_then(|key| column_map.get(key))
        .ok_or("Longitude not found")?;
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break Ok(out);
        }
        let row: Vec<&str> = buffer.trim().split(',').collect();
        let latitude = parse::parse_float(row[latitude_index]);
        let longitude = parse::parse_float(row[longitude_index]);
        if let Some(point) = longitude.zip(latitude) {
            out.push(point);
        }
    }
}

fn nonna_line(input: &str) -> Option<Point> {
    let mut split = input.split(' ');
    let latitude = parse::parse_dms_latitude(split.next()?)?;
    let longitude = parse::parse_dms_longitude(split.next()?)?;
    Some((longitude, latitude))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_point_lines() {
        let source = "0-0-0.0N 0-0-0.0E";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader);
        let expected = vec![(0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_multiple_point_lines() {
        let source = "0-0-0.0N 0-0-0.0E\n0-0-0.0N 0-0-0.0E";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader);
        let expected = vec![(0.0, 0.0), (0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn rejects_non_point_lines() {
        let source = "not actual point\n0-0-0.0N 0-0-0.0E";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader);
        let expected = vec![(0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_csv_longitude_first() {
        let source = "longitude,latitude\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_csv_latitude_first() {
        let source = "latitude,longitude\n49.58,-123.456";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_csv_short_names() {
        let source = "lon,lat\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_csv_capitalized_names() {
        let source = "Longitude,Latitiude\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_csv_despite_formatting() {
        let source = "\"longitude (float)\", \"latitiude (float)\"\n-123.456,49.58";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_csv_with_other_columns() {
        let source = "elevation,longitude,depth,latitiude,noise\n0.0,-123.456,0.0,49.58,0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader);
        let expected = vec![(-123.456, 49.58)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
