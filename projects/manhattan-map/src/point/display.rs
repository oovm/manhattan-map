use crate::SPoint;
use super::*;

impl Debug for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Axial")
            .field("q", &self.q)
            .field("s", &(-self.q - self.r))
            .field("r", &self.r)
            .finish()
    }
}

impl Display for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.q)
            .field(&(-self.q - self.r))
            .field(&self.r)
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
