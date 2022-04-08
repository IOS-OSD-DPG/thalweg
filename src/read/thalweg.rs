use crate::bathymetry::Bathymetry;
use crate::parse;
use crate::read::bathymetry;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

use json;

/// Read thalweg data from the NONNA-10 ASCII format
pub fn from_nonna<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    bathymetry::from_nonna(input)
}

/// Read thalweg data from a CSV
pub fn from_csv<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
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
    let depth_index = *column_map
        .keys()
        .find(|name| name.to_lowercase().starts_with("depth"))
        .and_then(|key| column_map.get(key))
        .ok_or("Depth not found")?;
    loop {
        buffer.clear();
        if input.read_line(&mut buffer)? == 0 {
            break Ok(out);
        }
        let row: Vec<&str> = buffer.split(',').collect();
        let latitude = parse::parse_float(row[latitude_index]);
        let longitude = parse::parse_float(row[longitude_index]);
        let depth = parse::parse_float(row[depth_index]);
        if let Some(((lat, lon), dep)) = latitude.zip(longitude).zip(depth) {
            out.push(Bathymetry::new(lat, lon, dep));
        }
    }
}

/// Read thalweg data from GeoJSON
pub fn from_geojson<T: Read>(input: &mut BufReader<T>) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let geojson = json::parse(&buffer)?;
    if geojson["type"] == "FeatureCollection" {
        from_feature_collection(&geojson)
    } else if geojson["type"] == "Feature" {
        from_feature(&geojson)
    } else {
        from_line_string(&geojson)
    }
}

fn from_feature_collection(input: &json::JsonValue) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    from_feature(&input["features"][0])
}

fn from_feature(input: &json::JsonValue) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    from_line_string(&input["geometry"])
}

fn from_line_string(input: &json::JsonValue) -> Result<Vec<Bathymetry>, Box<dyn Error>> {
    let mut out = vec![];
    if input["type"] != "LineString" {
        return Err(Box::<dyn Error>::from("Thalweg can only be constructed from a LineString object"));
    }
    let coordinates = &input["coordinates"];
    if coordinates.is_null() {
        return Err(Box::<dyn Error>::from("No coordinates found"));
    }
    for member in coordinates.members() {
        let longitude = member[0].as_f64().ok_or("Missing longitude")?;
        let latitude = member[1].as_f64().ok_or("Missing latitude")?;
        let elevation = member[2].as_f64().ok_or("Missing elevation")?;
        out.push(Bathymetry::new(latitude, longitude, -1.0 * elevation));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_thalweg_from_nonna() {
        let source = "0-0-0.0N 0-0-0.0E 0.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_nonna(&mut reader);
        let expected = vec![Bathymetry::new(0.0, 0.0, 0.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_thalweg_with_multiple_lines() {
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
    fn reads_thalweg_from_csv() {
        let source = "longitude,latitude,depth\n-123.456,49.58,100.0";
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_csv(&mut reader);
        let expected = vec![Bathymetry::new(49.58, -123.456, 100.0)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_thalweg_from_geoson_feature_collection() {
        let source = r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{},"geometry":{"type":"LineString","coordinates":[[-122.882765,49.46419,-9.144]]}}]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        let expected = vec![Bathymetry::new(49.46419, -122.882765, 9.144)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_thalweg_from_geoson_feature() {
        let source = r#"{"type":"Feature","properties":{},"geometry":{"type":"LineString","coordinates":[[-122.882765,49.46419,-9.144]]}}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        let expected = vec![Bathymetry::new(49.46419, -122.882765, 9.144)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn reads_thalweg_from_geoson_line_string() {
        let source = r#"{"type":"LineString","coordinates":[[-122.882765,49.46419,-9.144]]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        let expected = vec![Bathymetry::new(49.46419, -122.882765, 9.144)];
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn rejects_thalweg_from_geojson_wrong_type() {
        let source = r#"{"type":"Point","coordinates":[-122.882765,49.46419,-9.144]}"#;
        let mut reader = BufReader::new(source.as_bytes());
        let actual = from_geojson(&mut reader);
        assert!(actual.is_err());
    }
}
