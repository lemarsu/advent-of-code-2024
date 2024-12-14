use std::{error::Error, fs};

use crate::{robot::Robot, Size};

pub fn main(height: usize, width: usize, file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid_size = Size::new(height, width);
    let mut robots = Robot::parse(&content);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.move_next(&grid_size);
        }
    }

    let mut quadrants: [Vec<_>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for robot in robots.into_iter() {
        if let Some(quadrant) = grid_size.quadrant(&robot.pos) {
            quadrants[quadrant].push(robot);
        }
    }

    let sum = quadrants.iter().fold(1, |acc, q| {
        acc * q.len()
    });

    Ok(sum)
}
