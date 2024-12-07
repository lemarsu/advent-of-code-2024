use std::{collections::HashSet, error::Error, fs};

use crate::{direction::Direction::Top, grid::Grid, guardian::Guardian};

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid = Grid::new(&content);

    let mut guardian = Guardian::new(grid.find_char('^').unwrap(), Top);
    let start_pos = guardian.clone();
    let mut path_positions = HashSet::new();

    loop {
        let forward = guardian.pos.next(guardian.direction);
        if !grid.is_valid_point(&forward) {
            break;
        }
        if grid.get_at(&forward) == '#' {
            guardian.turn_right();
        } else {
            guardian.move_next();
        }
        path_positions.insert(guardian.pos.clone());
    }

    println!("Positions: {}", path_positions.len());

    let ret = path_positions.iter().fold(0, |acc, pos| {
        acc + if grid.could_loop_on_this_point(&start_pos, &pos) {
            1
        } else {
            0
        }
    });

    Ok(ret)
}
