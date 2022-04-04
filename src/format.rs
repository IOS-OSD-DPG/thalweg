use crate::bathymetry::Bathymetry;

use std::default;
use std::fmt;
use std::str;

#[derive(Clone, Copy, Debug)]
pub enum OutputFormat {
    Csv,
    Dms,
    GeoJson,
}

impl str::FromStr for OutputFormat {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(OutputFormat::Csv),
            "dms" => Ok(OutputFormat::Dms),
            "geojson" => Ok(OutputFormat::GeoJson),
            _ => Err("unrecognized output format"),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Csv => write!(f, "csv"),
            OutputFormat::Dms => write!(f, "dms"),
            OutputFormat::GeoJson => write!(f, "geojson"),
        }
    }
}

impl default::Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Dms
    }
}

pub fn extension(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::Csv => "csv",
        OutputFormat::Dms => "txt",
        OutputFormat::GeoJson => "geojson",
    }
}

pub fn convert(format: OutputFormat, input: &[Bathymetry]) -> String {
    match format {
        OutputFormat::Csv => to_csv(input),
        OutputFormat::Dms => to_dms(input),
        OutputFormat::GeoJson => to_geojson(input),
    }
}

fn to_csv(input: &[Bathymetry]) -> String {
    let mut out = "longitude,latitude,depth\n".to_string();
    for point in input {
        let (lon, lat) = point.point();
        let depth = point.depth();
        let line = format!("{lon},{lat},{depth}\n");
        out += &line;
    }
    out
}

fn to_dms(input: &[Bathymetry]) -> String {
    let mut out = "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n".to_string();
    for point in input {
        out += point.to_string().as_str();
        out += "\n";
    }
    out
}

fn to_geojson(input: &[Bathymetry]) -> String {
    let mut elems = input.iter().map(|b| {
        let (lon, lat) = b.point();
        format!("[{},{},{}]", lon, lat, -b.depth())
    });
    let mut joined = String::new();
    if let Some(elem) = elems.next() {
        joined += elem.as_str();
    }
    for elem in elems {
        joined += ",";
        joined += elem.as_str();
    }
    String::from("{")
        + "\"type\":\"FeatureCollection\",\"features\":[{"
        + "\"type\":\"Feature\",\"properties\":{},\"geometry\":{"
        + "\"type\":\"LineString\",\"coordinates\":["
        + joined.as_str()
        + "]"
        + "}"
        + "}]"
        + "}"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_dms_no_value() {
        let expected = "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n";
        assert_eq!(convert(OutputFormat::Dms, &[]), expected);
    }

    #[test]
    fn to_dms_one_value() {
        let input = Bathymetry::new(0.0, 0.0, 0.0);
        let expected = concat!(
            "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n",
            "00-00-0.000N 00-00-0.000E 0.000\n"
        );
        assert_eq!(convert(OutputFormat::Dms, &[input]), expected);
    }

    #[test]
    fn to_dms_many_values() {
        let input = Bathymetry::new(0.0, 0.0, 0.0);
        let expected = concat!(
            "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n",
            "00-00-0.000N 00-00-0.000E 0.000\n",
            "00-00-0.000N 00-00-0.000E 0.000\n"
        );
        assert_eq!(
            convert(OutputFormat::Dms, &[input.clone(), input]),
            expected
        );
    }

    #[test]
    fn to_geojson_no_value() {
        let expected = concat!(
            "{\"type\":\"FeatureCollection\",\"features\":[",
            "{\"type\":\"Feature\",\"properties\":{},\"geometry\":",
            "{\"type\":\"LineString\",\"coordinates\":[]}",
            "}]",
            "}"
        );
        assert_eq!(convert(OutputFormat::GeoJson, &[]), expected);
    }

    #[test]
    fn to_geojson_one_value() {
        let a = Bathymetry::new(48.7, -123.7, 100.4);
        let expected = concat!(
            "{",
            "\"type\":\"FeatureCollection\",",
            "\"features\":[",
            "{",
            "\"type\":\"Feature\",",
            "\"properties\":{},",
            "\"geometry\":{",
            "\"type\":\"LineString\",",
            "\"coordinates\":[",
            "[-123.7,48.7,-100.4]",
            "]",
            "}",
            "}",
            "]",
            "}"
        );
        assert_eq!(convert(OutputFormat::GeoJson, &[a]), expected);
    }

    #[test]
    fn to_geojson_many_values() {
        let a = Bathymetry::new(48.7, -123.7, 100.4);
        let b = Bathymetry::new(49.7, -123.7, 100.4);
        let expected = concat!(
            "{",
            "\"type\":\"FeatureCollection\",",
            "\"features\":[",
            "{",
            "\"type\":\"Feature\",",
            "\"properties\":{},",
            "\"geometry\":{",
            "\"type\":\"LineString\",",
            "\"coordinates\":[",
            "[-123.7,48.7,-100.4],",
            "[-123.7,49.7,-100.4]",
            "]",
            "}",
            "}",
            "]",
            "}"
        );
        assert_eq!(convert(OutputFormat::GeoJson, &[a, b]), expected);
    }

    #[test]
    fn to_csv_one_value() {
        let a = Bathymetry::new(48.7, -123.7, 100.4);
        let expected = "longitude,latitude,depth\n-123.7,48.7,100.4\n";
        assert_eq!(convert(OutputFormat::Csv, &[a]), expected);
    }

    #[test]
    fn to_csv_many_values() {
        let a = Bathymetry::new(48.7, -123.7, 100.4);
        let b = Bathymetry::new(49.7, -123.7, 100.4);
        let expected = concat!(
            "longitude,latitude,depth\n",
            "-123.7,48.7,100.4\n",
            "-123.7,49.7,100.4\n"
        );
        assert_eq!(convert(OutputFormat::Csv, &[a, b]), expected);
    }
}
