use crate::Direction;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Joint {
    pub fn new(x: isize, y: isize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
    pub fn from_point(x1: isize, y1: isize, x2: isize, y2: isize) -> Option<Self> {
        if x1 == x2 {
            if y1 == y2 + 1 {
                return Some(Self::new(x1, y1, Direction::Y(false)));
            }
            else if y1 == y2 - 1 {
                return Some(Self::new(x1, y1, Direction::Y(true)));
            }
        }
        else if y1 == y2 {
            if x1 == x2 + 1 {
                return Some(Self::new(x1, y1, Direction::X(false)));
            }
            else if x1 == x2 - 1 {
                return Some(Self::new(x1, y1, Direction::X(true)));
            }
        }
        None
    }

    pub fn source(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    pub fn target(&self) -> (isize, isize) {
        match self.direction {
            Direction::X(true) => (self.x + 1, self.y),
            Direction::X(false) => (self.x - 1, self.y),
            Direction::Y(true) => (self.x, self.y + 1),
            Direction::Y(false) => (self.x, self.y - 1),
        }
    }
}
