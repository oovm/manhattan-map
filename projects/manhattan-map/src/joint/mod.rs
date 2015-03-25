use crate::{point::AxialPoint, Direction};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    pub point: AxialPoint,
    pub direction: Direction,
}

impl Joint {
    pub fn new(point: &AxialPoint, direction: Direction) -> Self {
        Self { point: *point, direction }
    }

    pub fn source(&self) -> AxialPoint {
        self.point
    }
    pub fn target(&self) -> AxialPoint {
        self.point.go(self.direction)
    }
}
