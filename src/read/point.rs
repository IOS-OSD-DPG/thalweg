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

fn nonna_line(input: &str) -> Option<Point> {
    let mut split = input.split(' ');
    let latitude = parse::parse_dms_latitude(split.next()?)?;
    let longitude = parse::parse_dms_longitude(split.next()?)?;
    Some((longitude, latitude))
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

pub fn from_geojson<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let geojson = json::parse(&buffer)?;
    if geojson["type"] == "FeatureCollection" {
        from_feature_collection(&geojson)
    } else if geojson["type"] == "Feature" {
        from_features(&[&geojson])
    } else {
        from_points(&[&geojson])
    }
}

fn from_feature_collection(input: &json::JsonValue) -> Result<Vec<Point>, Box<dyn Error>> {
    let coll: Vec<&json::JsonValue> = input["features"].members().collect();
    from_features(&coll)
}

fn from_features(input: &[&json::JsonValue]) -> Result<Vec<Point>, Box<dyn Error>> {
    let coll: Vec<&json::JsonValue> = input.iter().map(|item| &item["geometry"]).collect();
    from_points(&coll)
}

fn from_points(input: &[&json::JsonValue]) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut out = vec![];
    if input.iter().any(|item| item["type"] != "Point") {
        return Err(Box::<dyn Error>::from(
            "Points can only be constructed from Point objects"
        ));
    }
    for point in input {
        let coords = &point["coordinates"];
        if coords.is_null() {
            continue;
        }
        let longitude = coords[0].as_f64().ok_or("Missing longitude")?;
        let latitude = coords[1].as_f64().ok_or("Missing latitude")?;
        out.push((longitude, latitude));
    }
    Ok(out)
}

pub fn from_geojson_line<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let geojson = json::parse(&buffer)?;
    if geojson["type"] == "FeatureCollection" {
        line_from_feature_collection(&geojson)
    } else if geojson["type"] == "Feature" {
        line_from_features(&geojson)
    } else {
        points_from_line(&geojson)
    }
}

fn line_from_feature_collection(input: &json::JsonValue) -> Result<Vec<Point>, Box<dyn Error>> {
    line_from_features(&input["features"][0])
}

fn line_from_features(input: &json::JsonValue) -> Result<Vec<Point>, Box<dyn Error>> {
    points_from_line(&input["geometry"])
}

fn points_from_line(input: &json::JsonValue) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut out = vec![];
    if input["type"] != "LineString" {
        return Err(Box::<dyn Error>::from(
            "Thalweg can only be constructed from a LineString object",
        ));
    }
    let coordinates = &input["coordinates"];
    if coordinates.is_null() {
        return Err(Box::<dyn Error>::from("No coordinates found"));
    }
    for member in coordinates.members() {
        let longitude = member[0].as_f64().ok_or("Missing longitude")?;
        let latitude = member[1].as_f64().ok_or("Missing latitude")?;
        out.push((longitude, latitude));
    }
    Ok(out)
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

    #[test]
    fn reads_point_geojson_just_one_point() {
        let source = r#"{"type":"Point","coordinates":[-123.4,54.3]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        let expected = vec![(-123.4, 54.3)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_geojson_just_one_feature() {
        let source = r#"{"type":"Feature","properties":{},"geometry":{"type":"Point","coordinates":[-123.4,54.3]}}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        let expected = vec![(-123.4, 54.3)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_point_geojson_full_collection() {
        let source = r#"{"type":"FeatureCollection","features":[
            {"type":"Feature","properties":{},"geometry":{"type":"Point","coordinates":[-123.4,54.3]}},
            {"type":"Feature","properties":{},"geometry":{"type":"Point","coordinates":[-123.3,54.4]}}
        ]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        let expected = vec![(-123.4, 54.3),(-123.3, 54.4)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn rejects_points_from_geojson_wrong_type() {
        let source = r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{},"geometry":{"type":"LineString","coordinates":[[-122.882765,49.46419,-9.144]]}}]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        assert!(actual.is_err());
    }

    #[test]
    fn reads_points_from_geoson_line_feature_collection() {
        let source = r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{},"geometry":{"type":"LineString","coordinates":[[-122.882765,49.46419,-9.144]]}}]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson_line(&mut reader);
        let expected = vec![(-122.882765, 49.46419)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_points_from_geoson_line_feature() {
        let source = r#"{"type":"Feature","properties":{},"geometry":{"type":"LineString","coordinates":[[-122.882765,49.46419,-9.144]]}}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson_line(&mut reader);
        let expected = vec![(-122.882765, 49.46419)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_points_from_geoson_line_string() {
        let source = r#"{"type":"LineString","coordinates":[[-122.882765,49.46419,-9.144]]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson_line(&mut reader);
        let expected = vec![(-122.882765, 49.46419)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn rejects_points_from_geojson_line_wrong_type() {
        let source = r#"{"type":"Point","coordinates":[-122.882765,49.46419,-9.144]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson_line(&mut reader);
        assert!(actual.is_err());
    }
}
