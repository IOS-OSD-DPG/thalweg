use geo::Point;

#[derive(Debug)]
pub struct Bathymetry {
    location: Point<f64>,
    depth: f64,
}

impl Bathymetry {
    pub fn new(location: Point<f64>, depth: f64) -> Self {
        Self { location, depth }
    }

    pub fn location(&self) -> Point<f64> {
        self.location
    }

    pub fn depth(&self) -> f64 {
        self.depth
    }
}
