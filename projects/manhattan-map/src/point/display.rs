
use super::*;

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normed = s.to_lowercase();
        match normed.as_str() {
            "east" | "right" | "→" => Ok(Direction::X(true)),
            "west" | "left" | "←" => Ok(Direction::X(false)),
            "north" | "up" | "↑" => Ok(Direction::Y(true)),
            "south" | "down" | "↓" => Ok(Direction::Y(false)),
            _ => Err(normed),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::X(true) => { f.write_str("→") }
            Direction::X(false) => { f.write_str("←") }
            Direction::Y(true) => { f.write_str("↑") }
            Direction::Y(false) => { f.write_str("↓") }
        }
    }
}

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
