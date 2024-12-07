#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Top,
    Left,
    Bottom,
    Right,
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

impl Direction {
    pub fn has_left(&self) -> bool {
        matches!(self, Self::Left | Self::TopLeft | Self::BottomLeft)
    }

    pub fn has_right(&self) -> bool {
        matches!(self, Self::Right | Self::TopRight | Self::BottomRight)
    }

    pub fn has_top(&self) -> bool {
        matches!(self, Self::Top | Self::TopLeft | Self::TopRight)
    }

    pub fn has_bottom(&self) -> bool {
        matches!(self, Self::Bottom | Self::BottomLeft | Self::BottomRight)
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
            Self::Left => Self::Top,
            Self::TopRight => Self::BottomRight,
            Self::BottomRight => Self::BottomLeft,
            Self::BottomLeft => Self::TopLeft,
            Self::TopLeft => Self::TopRight,
        }
    }
}
