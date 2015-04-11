use crate::Direction;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    pub x: isize,
    pub y: isize,
    pub direction: Direction,
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
    pub fn new(x: isize, y: isize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
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
