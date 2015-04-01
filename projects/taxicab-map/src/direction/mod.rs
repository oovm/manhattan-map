use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

mod display;
mod convert;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// - `S-false means left`
    X(bool),
    /// - `S-true means right`
    Y(bool),
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::X(true),
            Direction::X(false),
            Direction::Y(true),
            Direction::Y(false),
        ]
    }
}
