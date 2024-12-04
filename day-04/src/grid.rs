use crate::{direction::Direction, point::Point};

pub struct Grid {
    content: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(str: &str) -> Self {
        let content: Vec<Vec<_>> = str
            .split("\n")
            .map(|line| line.chars().collect())
            .filter(|chars: &Vec<char>| !chars.is_empty())
            .collect();

        Self { content }
    }

    fn get_at(&self, pos: &Point) -> char {
        self.get(pos.x, pos.y)
    }

    fn get(&self, x: i32, y: i32) -> char {
        self.content[y as usize][x as usize]
    }

    pub fn match_at_point_in_direction(
        &self,
        input: &Vec<char>,
        p: &Point,
        direction: Direction,
    ) -> bool {
        let mut p = p.clone();
        let mut i = 0;
        while self.is_valid_point(&p) && i < input.len() && self.get_at(&p) == input[i] {
            i += 1;
            p.move_next(direction);
        }
        i == input.len()
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
