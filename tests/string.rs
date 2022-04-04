use thalweg::bathymetry::Bathymetry;
use thalweg::format::{self, OutputFormat};
use thalweg::read;

use std::io::BufReader;

#[test]
fn geojson_parse_unparse_equivalent() {
    let input = vec![
        Bathymetry::new(0.0, 0.0, 0.0),
        Bathymetry::new(49.58, -123.456, 100.0),
    ];
    let string = format::convert(OutputFormat::GeoJson, &input);
    let mut reader = BufReader::new(string.as_bytes());
    let actual = read::thalweg::from_geojson(&mut reader);
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), input);
}

#[test]
fn csv_parse_unparse_equivalent() {
    let input = vec![
        Bathymetry::new(0.0, 0.0, 0.0),
        Bathymetry::new(49.58, -123.456, 100.0),
    ];
    let string = format::convert(OutputFormat::Csv, &input);
    let mut reader = BufReader::new(string.as_bytes());
    let actual = read::thalweg::from_csv(&mut reader);
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), input);
}

#[test]
fn nonna_parse_unparse_equivalent() {
    let input = vec![
        Bathymetry::new(0.0, 0.0, 0.0),
        Bathymetry::new(49.58, -123.456, 100.0),
    ];
    let string = format::convert(OutputFormat::Dms, &input);
    let mut reader = BufReader::new(string.as_bytes());
    let actual = read::thalweg::from_nonna(&mut reader);
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), input);
}
