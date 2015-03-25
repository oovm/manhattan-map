use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WPoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub x: isize,
    /// S-axis index, X-axis index in cube coordinates
    pub y: isize,
}

impl WPoint {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn go(&self, direction: Direction) -> Self {
        <WPoint as Into<AxialPoint>>::into(*self).go(direction).into()
    }
}

impl From<AxialPoint> for WPoint {
    fn from(point: AxialPoint) -> Self {
        WPoint::new(point.q, point.r)
    }
}

impl Into<AxialPoint> for WPoint {
    fn into(self) -> AxialPoint {
        AxialPoint::new(self.x, self.y)
    }
}
