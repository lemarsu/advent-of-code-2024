use regex::Regex;

use crate::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    pub pos: Point,
    pub velocity: Point,
}

impl Robot {
    pub fn new(pos: Point, velocity: Point) -> Self {
        Self { pos, velocity }
    }

    pub fn parse(str: &str) -> Vec<Self> {
        let line_re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        str.split("\n")
            .filter_map(|line| line_re.captures(line))
            .map(|captures| {
                let (_, [x, y, vx, vy]) = captures.extract();
                Robot::new(
                    Point::new(x.parse().unwrap(), y.parse().unwrap()),
                    Point::new(vx.parse().unwrap(), vy.parse().unwrap()),
                )
            })
            .collect()
    }

    pub fn move_next(&mut self, grid_size: &Size) {
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;

        self.warp(grid_size);
    }

    pub fn warp(&mut self, grid_size: &Size) {
        if self.pos.x < 0 {
            self.pos.x += grid_size.width as i32;
        } else if self.pos.x >= grid_size.width as i32 {
            self.pos.x %= grid_size.width as i32;
        }
        if self.pos.y < 0 {
            self.pos.y += grid_size.height as i32;
        } else if self.pos.y >= grid_size.height as i32 {
            self.pos.y %= grid_size.height as i32;
        }
    }
}
