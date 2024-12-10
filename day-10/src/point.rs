use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

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
        use Direction::*;

        if matches!(direction, Left) {
            self.x -= 1;
        }
        if matches!(direction, Right) {
            self.x += 1;
        }
        if matches!(direction, Top) {
            self.y -= 1;
        }
        if matches!(direction, Bottom) {
            self.y += 1;
        }
    }

    pub fn next(&self, direction: Direction) -> Self {
        let mut p = self.clone();
        p.move_next(direction);
        p
    }
}

impl From<i32> for Point {
    fn from(value: i32) -> Self {
        Self::new(value, value)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_add_test() {
        let p1 = Point::new(1, 3);
        let p2 = Point::new(-5, 7);
        let p3 = Point::new(8, -13);
        let p4 = Point::new(-6, 17);

        assert_eq!(p1 + p2, Point::new(-4, 10));
        assert_eq!(p1 + p3, Point::new(9, -10));
        assert_eq!(p1 + p4, Point::new(-5, 20));
    }
}
