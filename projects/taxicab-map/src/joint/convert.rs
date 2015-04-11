use super::*;

impl Neg for Joint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        !self
    }
}

impl Not for Joint {
    type Output = Self;

    fn not(self) -> Self::Output {
        Joint::new(self.x, self.y, !self.direction)
    }
}
