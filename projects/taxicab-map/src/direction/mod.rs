use crate::Joint;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Neg, Not},
    str::FromStr,
};

mod convert;
mod display;

/// Represents one of 4 directions over a taxicab map.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// - `S-false means left`
    X(bool),
    /// - `S-true means right`
    Y(bool),
}

impl Direction {
    /// All 4 directions over a taxicab map.
    pub fn all() -> [Direction; 4] {
        [Direction::X(true), Direction::X(false), Direction::Y(true), Direction::Y(false)]
    }
    /// Create a joint from a point and this direction.
    pub fn as_joint(&self, x: isize, y: isize) -> Joint {
        Joint::new(x, y, *self)
    }
}
