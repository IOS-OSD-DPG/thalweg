use crate::bathymetry::Bathymetry;
use crate::parse;

use std::io::{self, BufRead, BufReader, Read};

use geo::Point;

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

pub fn read_corner_lines<T: Read>(input: &mut BufReader<T>) -> io::Result<Vec<Point<f64>>> {
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
    Some(Bathymetry::new(Point::new(longitude, latitude), depth))
}

fn read_corner_line(input: &str) -> Option<Point<f64>> {
    let mut split = input.split(' ');
    let latitude = parse::parse_dms_latitude(split.next()?)?;
    let longitude = parse::parse_dms_longitude(split.next()?)?;
    Some(Point::new(longitude, latitude))
}
