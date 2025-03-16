#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub lng: f64,
    pub lat: f64,
}

impl Coordinates {
    pub fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    min_coord: Coordinates,
    max_coord: Coordinates,
}

impl Rect {
    pub fn new(min_coord: Coordinates, max_coord: Coordinates) -> Self {
        Self {
            min_coord,
            max_coord,
        }
    }

    pub fn min(&self) -> Coordinates {
        self.min_coord
    }

    pub fn max(&self) -> Coordinates {
        self.max_coord
    }
}
