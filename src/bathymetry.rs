use geo;
use geo::algorithm::haversine_distance::HaversineDistance;

use rstar::{PointDistance, RTreeObject};

pub type Point = (f64, f64);
pub type Location = (isize, isize);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bathymetry {
    location: Location,
    depth: f64,
}

const PRECISION: f64 = 1000000.0;

impl Bathymetry {
    pub fn new(latitude: f64, longitude: f64, depth: f64) -> Self {
        Self {
            location: ((longitude * PRECISION) as isize, (latitude * PRECISION) as isize),
            depth,
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn point(&self) -> Point {
        let (longitude, latitude) = self.location;
        ((longitude as f64) / PRECISION, (latitude as f64) / PRECISION)
    }

    pub fn depth(&self) -> f64 {
        self.depth
    }
}

impl RTreeObject for Bathymetry {
    type Envelope = <Point as RTreeObject>::Envelope;

    fn envelope(&self) -> Self::Envelope {
        self.point().envelope()
    }
}

impl PointDistance for Bathymetry {
    fn distance_2(&self, &point: &(f64, f64)) -> f64 {
        let distance = geo::Point::from(self.point()).haversine_distance(&geo::Point::from(point));
        distance * distance
    }
}
