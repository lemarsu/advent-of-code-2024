use crate::{direction::Direction, point::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guardian {
    pub pos: Point,
    pub direction: Direction,
}

impl Guardian {
    pub fn new(pos: Point, direction: Direction) -> Self {
        Self { pos, direction }
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    pub fn move_next(&mut self) {
        self.pos.move_next(self.direction);
    }

    pub fn next_pos(&self) -> Point {
        self.pos.next(self.direction)
    }
}
