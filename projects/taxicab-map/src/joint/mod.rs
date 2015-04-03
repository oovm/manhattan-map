use crate::{Direction};
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
