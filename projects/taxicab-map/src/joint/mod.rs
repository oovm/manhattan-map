use crate::Direction;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
mod convert;
use std::ops::{Neg, Not};

/// A point with the direction.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Debug for Joint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Joint").field("x", &self.x).field("y", &self.y).field("direction", &self.direction.to_string()).finish()
    }
}

impl Display for Joint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Joint").field(&self.x).field(&self.y).field(&self.direction.to_string()).finish()
    }
}

impl Joint {
    /// Create a new joint from a point and a direction.
    pub fn new(x: isize, y: isize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
    /// Create a new joint from two points.
    pub fn from_point((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> Self {
        if x1 == x2 {
            if y1 == y2 + 1 {
                return Self::new(x1, y1, Direction::Y(false));
            }
            else if y1 == y2 - 1 {
                return Self::new(x1, y1, Direction::Y(true));
            }
        }
        else if y1 == y2 {
            if x1 == x2 + 1 {
                return Self::new(x1, y1, Direction::X(false));
            }
            else if x1 == x2 - 1 {
                return Self::new(x1, y1, Direction::X(true));
            }
        }
        panic!("({},{}) and ({},{}) are not adjacent", x1, y1, x2, y2);
    }
    /// Get the direction of the joint.
    pub fn get_direction(&self) -> Direction {
        self.direction
    }
    /// Set the direction of the joint.
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    /// Get the source point of the joint.
    pub fn source(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    /// Get the target point of the joint.
    pub fn target(&self) -> (isize, isize) {
        match self.direction {
            Direction::X(true) => (self.x + 1, self.y),
            Direction::X(false) => (self.x - 1, self.y),
            Direction::Y(true) => (self.x, self.y + 1),
            Direction::Y(false) => (self.x, self.y - 1),
        }
    }
}
