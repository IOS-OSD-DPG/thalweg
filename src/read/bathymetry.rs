use crate::bathymetry::Bathymetry;
use crate::parse;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

use geo::{Coordinate, Point, Polygon};
use geo::algorithm::contains::Contains;

/// Read bathymetry data from the NONNA-10 ASCII format
pub fn from_nonna<T: Read>(input: &mut BufReader<T>, bb: &Option<Polygon<f64>>) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    let mut out = vec![];
    let mut buffer = String::new();
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break Ok(out);
        }
        if let Some(value) = nonna_line(&buffer) {
            // use Option::iter to avoid consuming bb
            // Iterator::all returns true on empty iterator
            if bb.iter().all(|b| b.contains(&Coordinate::from(value.point()))) {
                out.push(value);
            }
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

pub fn from_csv<T: Read>(input: &mut BufReader<T>, bb: &Option<Polygon<f64>>) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
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
    let depth_index = column_map
        .keys()
        .find(|name| name.to_lowercase().starts_with("depth"))
        .and_then(|key| column_map.get(key))
        .map(|&value| value);
    let elevation_index = column_map
        .keys()
        .find(|name| name.to_lowercase().starts_with("elevation"))
        .and_then(|key| column_map.get(key))
        .map(|&value| value);
    if depth_index.is_none() && elevation_index.is_none() {
        return Err(Box::<dyn Error>::from("Depth not found".to_string()));
    }
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break Ok(out);
        }
        let row: Vec<&str> = buffer.split(',').collect();
        let latitude = parse::parse_float(row[latitude_index]);
        let longitude = parse::parse_float(row[longitude_index]);
        let depth = if let Some(index) = depth_index {
            parse::parse_float(row[index])
        } else if let Some(index) = elevation_index {
            parse::parse_float(row[index]).map(|value| value * -1.0)
        } else {
            unreachable!()
        };
        if let Some(((lat, lon), dep)) = latitude.zip(longitude).zip(depth) {
            // use Option::iter to avoid consuming bb
            // Iterator::all returns true on empty iterator
            if bb.iter().all(|b| b.contains(&Point::new(lon, lat))) {
                out.push(Bathymetry::new(lat, lon, dep));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_bathymetry_lines() {
        let source = "0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader, &None);
        let expected = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_multiple_bathymetry_lines() {
        let source = "0-0-0.0N 0-0-0.0E 0.0\n0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader, &None);
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
        let actual = from_nonna(&mut reader, &None);
        let expected = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_bathymetry_from_csv() {
        let source = "longitude,latitude,depth\n-123.456,49.58,100.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader, &None);
        let expected = vec![Bathymetry::new(49.58, -123.456, 100.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_bathymetry_from_csv_with_elevation() {
        let source = "longitude,latitude,elevation\n-123.456,49.58,-100.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader, &None);
        let expected = vec![Bathymetry::new(49.58, -123.456, 100.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
