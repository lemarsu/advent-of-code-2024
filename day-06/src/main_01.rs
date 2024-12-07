use std::{collections::HashSet, error::Error, fs};

use crate::{direction::Direction::Top, grid::Grid, guardian::Guardian, point::Point};

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid = Grid::new(&content);

    let mut guardian = Guardian::new(grid.find_char('^').unwrap(), Top);
    let mut positions: HashSet<Point> = HashSet::default();
    let mut moves = 0;
    let mut rotations = 0;

    loop {
        positions.insert(guardian.pos.clone());
        let next_pos = guardian.pos.next(guardian.direction);
        if !grid.is_valid_point(&next_pos) {
            break;
        }
        if grid.get_at(&next_pos) == '#' {
            guardian.turn_right();
            rotations += 1;
        } else {
            moves += 1;
            guardian.move_next();
        }
    }

    println!("Moves: {}", moves);
    println!("Rotations: {}", rotations);

    Ok(positions.len() as i32)
}
