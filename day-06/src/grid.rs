use std::collections::HashSet;

use crate::{
    direction::Direction::{Bottom, Right},
    guardian::Guardian,
    point::Point,
};

pub struct Grid {
    content: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(str: &str) -> Self {
        let content = str
            .split("\n")
            .map(|line| line.chars().collect())
            .filter(|chars: &Vec<char>| !chars.is_empty())
            .collect();

        Self { content }
    }

    pub fn get_at(&self, pos: &Point) -> char {
        self.get(pos.x, pos.y)
    }

    fn get(&self, x: i32, y: i32) -> char {
        self.content[y as usize][x as usize]
    }

    pub fn find_char(&self, char: char) -> Option<Point> {
        let mut p = Point::new(0, 0);
        while self.is_valid_point(&p) {
            while self.is_valid_point(&p) {
                if self.get_at(&p) == char {
                    return Some(p);
                }
                p.move_next(Right);
            }

            p.x = 0;
            p.move_next(Bottom);
        }
        None
    }

    pub fn could_loop_on_this_point(&self, start_pos: &Guardian, block: &Point) -> bool {
        let mut guardian = start_pos.clone();
        let mut previous = HashSet::new();
        previous.insert(guardian.clone());
        loop {
            let forward = guardian.next_pos();
            if !self.is_valid_point(&forward) || block == &start_pos.pos {
                return false;
            }
            if &forward == block || self.get_at(&forward) == '#' {
                guardian.turn_right();
            } else {
                guardian.move_next();
            }
            if previous.contains(&guardian) {
                return true;
            }
            previous.insert(guardian.clone());
        }
    }

    fn rows(&self) -> usize {
        self.content.len()
    }

    fn cols(&self) -> usize {
        if self.rows() > 0 {
            self.content[0].len()
        } else {
            0
        }
    }

    pub fn is_valid_point(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols() as i32 && point.y >= 0 && point.y < self.rows() as i32
    }
}
