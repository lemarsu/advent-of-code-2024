use crate::direction::Direction;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_next(&mut self, direction: Direction) {
        if direction.has_left() {
            self.x -= 1;
        }
        if direction.has_right() {
            self.x += 1;
        }
        if direction.has_top() {
            self.y -= 1;
        }
        if direction.has_bottom() {
            self.y += 1;
        }
    }

    pub fn next(&self, direction: Direction) -> Self {
        let mut p = self.clone();
        p.move_next(direction);
        p
    }
}
