use crate::direction::{Direction, BOTTOM, LEFT, RIGHT, TOP};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_next(&mut self, direction: Direction) {
        if direction & LEFT != 0 {
            self.x -= 1;
        }
        if direction & RIGHT != 0 {
            self.x += 1;
        }
        if direction & TOP != 0 {
            self.y -= 1;
        }
        if direction & BOTTOM != 0 {
            self.y += 1;
        }
    }

    pub fn next(&self, direction: Direction) -> Self {
        let mut p = self.clone();
        p.move_next(direction);
        p
    }
}
