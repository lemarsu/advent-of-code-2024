use crate::{Direction, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointBorder {
    point: Point,
    border: Direction,
}

impl PointBorder {
    pub fn new(point: Point, border: Direction) -> Self {
        Self { point, border }
    }
}
