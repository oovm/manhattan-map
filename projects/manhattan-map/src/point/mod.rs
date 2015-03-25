use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Display, Formatter};

pub mod s_point;
pub mod w_point;
pub mod h_point;

mod display;
mod convert;

/// A point in axial coordinates, standard form of a hexagon map
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AxialPoint {
    pub q: isize,
    pub r: isize,
}


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// - `S-true means right`
    /// - `S-false means left`
    S(bool),
    R(bool),
    Q(bool),
}

impl Direction {
    pub fn all() -> [Direction; 6] {
        [
            Direction::S(true),
            Direction::S(false),
            Direction::R(true),
            Direction::R(false),
            Direction::Q(true),
            Direction::Q(false),
        ]
    }
}


impl AxialPoint {
    /// Create a new point in axial coordinates
    pub fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }
    /// Create a new point in axial coordinates from pixel coordinates
    pub fn from_pixel(x: f64, y: f64, radius: f64) -> Self {
        let q = (x * 3.0f64.sqrt() / 3.0 - y / 3.0) / radius;
        let r = y * 2.0 / 3.0 / radius;
        Self::new(q.round() as isize, r.round() as isize)
    }
    /// Get the pixel coordinates of the center of the hexagon
    pub fn get_center(&self, radius: f64) -> (f64, f64) {
        let x = radius * 3.0f64.sqrt() * (self.q as f64 + self.r as f64 / 2.0);
        let y = radius * 3.0 / 2.0 * self.r as f64;
        (x, y)
    }
    /// Get the pixel coordinates of the corners of the hexagon
    pub fn get_corners(&self, radius: f64) -> [(f64, f64); 6] {
        let (center_x, center_y) = self.get_center(radius);
        let mut corners = [(0.0, 0.0); 6];
        for i in 0..6 {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / 6.0;
            corners[i] = (center_x + radius * angle.cos(), center_y + radius * angle.sin());
        }
        corners
    }
    pub fn nearby(&self) -> Vec<Self> {
        Direction::all()
            .iter()
            .map(|direction| self.go(*direction))
            .collect()
    }
}

impl AxialPoint {
    /// Get the pixel coordinates of the center of the hexagon
    pub fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::S(true) => Self::new(self.q + 1, self.r - 1),
            Direction::S(false) => Self::new(self.q - 1, self.r + 1),
            Direction::R(true) => Self::new(self.q + 1, self.r),
            Direction::R(false) => Self::new(self.q - 1, self.r),
            Direction::Q(true) => Self::new(self.q, self.r - 1),
            Direction::Q(false) => Self::new(self.q, self.r + 1),
        }
    }
    /// Calculate the euclidean distance between two points
    pub fn euclidean_distance(&self, other: &Self, radius: f64) -> f64 {
        let lhs = self.get_center(radius);
        let rhs = other.get_center(radius);
        ((lhs.0 - rhs.0).powi(2) + (lhs.1 - rhs.1).powi(2)).sqrt()
    }
    /// Calculate the manhattan distance between two points
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.q - other.q).abs() + (self.r - other.r).abs()) as usize
    }
}



