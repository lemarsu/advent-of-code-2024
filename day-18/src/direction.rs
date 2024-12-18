use std::str::FromStr;

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

    pub fn horizontal(&self) -> bool {
        matches!(self, Left | Right)
    }
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).unwrap() {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Top),
            'v' => Ok(Self::Bottom),
            _ => unreachable!(),
        }
    }
}
