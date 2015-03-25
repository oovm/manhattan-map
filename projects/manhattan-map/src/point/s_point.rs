use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SPoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub q: isize,
    /// S-axis index, X-axis index in cube coordinates
    pub s: isize,
    /// R-axis index, Y-axis index in cube coordinates
    pub r: isize,
}

impl SPoint {
    pub fn new(q: isize, s: isize, r: isize) -> Self {
        Self { q, s, r }
    }
    pub fn go(&self, direction: Direction) -> Self {
        <SPoint as Into<AxialPoint>>::into(*self).go(direction).into()
    }
}

impl From<AxialPoint> for SPoint {
    fn from(point: AxialPoint) -> Self {
        SPoint::new(point.q, -point.q - point.r, point.r)
    }
}

impl Into<AxialPoint> for SPoint {
    fn into(self) -> AxialPoint {
        AxialPoint::new(self.q, self.r)
    }
}
