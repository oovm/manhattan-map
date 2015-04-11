use super::*;

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        !self
    }
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::X(s) => Direction::X(!s),
            Direction::Y(s) => Direction::Y(!s),
        }
    }
}
