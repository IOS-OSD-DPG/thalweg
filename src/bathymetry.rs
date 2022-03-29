use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::hash::Hash;

use geo;
use geo::algorithm::haversine_distance::HaversineDistance;

use rstar::{PointDistance, RTreeObject};

pub type Point = (f64, f64);
pub type Location = (isize, isize);

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Bathymetry {
    location: Location,
    depth: isize,
}

const COORD_PRECISION: f64 = 1000000.0;
const DEPTH_PRECISION: f64 = 1000.0;

impl Bathymetry {
    pub fn new(latitude: f64, longitude: f64, depth: f64) -> Self {
        Self {
            location: (
                (longitude * COORD_PRECISION) as isize,
                (latitude * COORD_PRECISION) as isize,
            ),
            depth: (depth * DEPTH_PRECISION) as isize,
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn point(&self) -> Point {
        let (longitude, latitude) = self.location;
        (
            (longitude as f64) / COORD_PRECISION,
            (latitude as f64) / COORD_PRECISION,
        )
    }

    pub fn depth(&self) -> f64 {
        self.depth as f64 / DEPTH_PRECISION
    }

    pub fn distance_to(&self, other: &Bathymetry) -> f64 {
        self.distance_to_point(&other.point())
    }

    pub fn distance_to_point(&self, &other: &Point) -> f64 {
        geo::Point::from(self.point()).haversine_distance(&geo::Point::from(other))
    }
}

impl RTreeObject for Bathymetry {
    type Envelope = <Point as RTreeObject>::Envelope;

    fn envelope(&self) -> Self::Envelope {
        self.point().envelope()
    }
}

impl PointDistance for Bathymetry {
    fn distance_2(&self, point: &(f64, f64)) -> f64 {
        let distance = self.distance_to_point(point);
        distance * distance
    }
}

impl fmt::Display for Bathymetry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (longitude, latitude) = self.point();
        let lat_degrees = latitude.abs().trunc() as isize;
        let lat_minutes = ((latitude.abs() * 60.0).trunc() as isize) % 60;
        let lat_seconds = (latitude.abs() * 3600.0).rem_euclid(60.0);
        let ns = if latitude < 0.0 { 'S' } else { 'N' };
        let lon_degrees = longitude.abs().trunc() as isize;
        let lon_minutes = ((longitude.abs() * 60.0).trunc() as isize) % 60;
        let lon_seconds = (longitude.abs() * 3600.0).rem_euclid(60.0);
        let ew = if longitude < 0.0 { 'W' } else { 'E' };
        write!(
            f,
            "{}-{}-{:.3}{} {}-{}-{:.3}{} {}",
            lat_degrees,
            lat_minutes,
            lat_seconds,
            ns,
            lon_degrees,
            lon_minutes,
            lon_seconds,
            ew,
            self.depth
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to_self_is_zero() {
        let point = Bathymetry::new(0.0, 0.0, 0.0);
        assert_eq!(point.distance_to(&point), 0.0);
    }

    #[test]
    fn distance_between_points_is_positive() {
        let a = Bathymetry::new(0.0, 0.0, 0.0);
        let b = Bathymetry::new(1.0, 1.0, 1.0);
        assert!(a.distance_to(&b) > 0.0);
        assert!(b.distance_to(&a) > 0.0);
    }

    #[test]
    fn distance_between_points_is_reflexive() {
        let a = Bathymetry::new(0.0, 0.0, 0.0);
        let b = Bathymetry::new(1.0, 1.0, 1.0);
        assert_eq!(a.distance_to(&b), b.distance_to(&a));
    }

    #[test]
    fn distance_at_equator() {
        let start = Bathymetry::new(0.0, 0.0, 0.0);
        let end = Bathymetry::new(1.0, 1.0, 1.0);
        let distance_km = start.distance_to(&end) / 1000.0;
        // distance should be ~157km
        assert!((156.5..157.5).contains(&distance_km));
    }

    #[test]
    fn distance_in_vancouver() {
        let start = Bathymetry::new(49.4678, -122.883, 0.0);
        let end = Bathymetry::new(49.2989, -122.94, 0.0);
        let distance_km = start.distance_to(&end) / 1000.0;
        // distance should be ~19km
        assert!((18.5..19.5).contains(&distance_km));
    }

    #[test]
    fn depth_results_in_not_equal() {
        let a = Bathymetry::new(0.0, 0.0, 0.0);
        let b = Bathymetry::new(0.0, 0.0, 1.0);
        assert_ne!(a, b);
    }
}
