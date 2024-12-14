#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Top,
    Left,
    Bottom,
    Right,
}
use Direction::*;

impl Direction {
    pub fn all() -> [Self; 4] {
        use Direction::*;
        [Top, Right, Bottom, Left]
    }

    pub fn left(&self) -> Self {
        match self {
            Top => Left,
            Left => Bottom,
            Bottom => Right,
            Right => Top,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Left => Top,
            Top => Right,
            Right => Bottom,
            Bottom => Left,
        }
    }
}
