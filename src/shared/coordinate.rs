use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Coordinate) -> f64 {
        let x = self.x.abs_diff(other.x).pow(2);
        let y = self.y.abs_diff(other.y).pow(2);
        (x as f64).add(y as f64).sqrt()
    }

    pub fn axial_distance(&self, other: &Coordinate) -> usize {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);
        x + y
    }
}
