use crate::bathymetry::Bathymetry;

use std::default;
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum OutputFormat {
    Dms,
    GeoJson,
}

impl str::FromStr for OutputFormat {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dms" => Ok(OutputFormat::Dms),
            "geojson" | "gjs" => Ok(OutputFormat::GeoJson),
            _ => Err("unrecognized output format"),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

pub fn convert(format: OutputFormat, input: Vec<Bathymetry>) -> String {
    match format {
        OutputFormat::Dms => to_dms(input),
        OutputFormat::GeoJson => to_geojson(input),
    }
}

fn to_dms(input: Vec<Bathymetry>) -> String {
    let mut out = "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n".to_string();
    for point in input {
        out += point.to_string().as_str();
        out += "\n";
    }
    out
}

fn to_geojson(input: Vec<Bathymetry>) -> String {
    let mut elems = input.iter().map(|b| {
        let (lon, lat) = b.point();
        let depth = b.depth();
        String::from("{\"type\":\"Feature\"\"properties\":{\"depth\":")
            + depth.to_string().as_str()
            + concat!("}\"geometry\":{\"type\":\"Point\",\"coordinates\":[")
            + lon.to_string().as_str()
            + ","
            + lat.to_string().as_str()
            + "]}}"
    });
    let mut joined = String::new();
    if let Some(elem) = elems.next() {
        joined += elem.as_str();
    }
    for elem in elems {
        joined += ",";
        joined += elem.as_str();
    }
    format!(
        "{{\"type\":\"FeatureCollection\",\"features\":[{}]}}",
        joined
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_dms_no_value() {
        let expected = "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n";
        assert_eq!(convert(OutputFormat::Dms, vec![]), expected);
    }

    #[test]
    fn to_dms_one_value() {
        let input = Bathymetry::new(0.0, 0.0, 0.0);
        let expected = concat!(
            "\"Lat (DMS)\" \"Long (DMS)\" \"Depth (m)\"\n",
            "00-00-0.000N 00-00-0.000E 0.000\n"
        );
        assert_eq!(convert(OutputFormat::Dms, vec![input]), expected);
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
            convert(OutputFormat::Dms, vec![input.clone(), input.clone()]),
            expected
        );
    }

    #[test]
    fn to_geojson_no_value() {
        let expected = "{\"type\":\"FeatureCollection\",\"features\":[]}";
        assert_eq!(convert(OutputFormat::GeoJson, vec![]), expected);
    }

    #[test]
    fn to_geojson_one_value() {
        let input = Bathymetry::new(48.7, -123.7, 100.4);
        let expected = concat!(
            "{",
            "\"type\":\"FeatureCollection\",",
            "\"features\":[",
            "{",
            "\"type\":\"Feature\"",
            "\"properties\":{",
            "\"depth\":100.4",
            "}",
            "\"geometry\":{",
            "\"type\":\"Point\",",
            "\"coordinates\":[",
            "-123.7,48.7",
            "]",
            "}",
            "}",
            "]",
            "}"
        );
        assert_eq!(convert(OutputFormat::GeoJson, vec![input]), expected);
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
            "\"type\":\"Feature\"",
            "\"properties\":{",
            "\"depth\":100.4",
            "}",
            "\"geometry\":{",
            "\"type\":\"Point\",",
            "\"coordinates\":[",
            "-123.7,48.7",
            "]",
            "}",
            "},",
            "{",
            "\"type\":\"Feature\"",
            "\"properties\":{",
            "\"depth\":100.4",
            "}",
            "\"geometry\":{",
            "\"type\":\"Point\",",
            "\"coordinates\":[",
            "-123.7,49.7",
            "]",
            "}",
            "}",
            "]",
            "}"
        );
        assert_eq!(convert(OutputFormat::GeoJson, vec![a, b]), expected);
    }
}
