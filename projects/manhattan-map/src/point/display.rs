use crate::SPoint;
use super::*;

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Axial")
            .field("q", &self.x)
            .field("s", &(-self.x - self.y))
            .field("r", &self.y)
            .finish()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&(-self.x - self.y))
            .field(&self.y)
            .finish()
    }
}

impl Debug for SPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StepPoint")
            .field("s", &self.s)
            .field("q", &self.q)
            .field("r", &self.r)
            .finish()
    }
}

impl Display for SPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StepPoint")
            .field(&self.s)
            .field(&self.q)
            .field(&self.r)
            .finish()
    }
}
