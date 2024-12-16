use crate::{Direction, Point};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Raindeer {
    pub pos: Point,
    pub direction: Direction,
}

impl Raindeer {
    pub fn new(pos: Point, direction: Direction) -> Self {
        Self { pos, direction }
    }

    pub fn move_next(&mut self) {
        self.pos.move_next(self.direction);
    }
}
