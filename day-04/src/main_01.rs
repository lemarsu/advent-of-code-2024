use std::{error::Error, fs};

use crate::{
    direction::{BOTTOM, LEFT, RIGHT, TOP},
    grid::Grid,
    point::Point,
};

fn count_all_occurences(grid: &Grid, input: &str) -> i32 {
    let input: Vec<_> = input.chars().collect();

    let mut ret = 0;

    let mut p = Point::new(0, 0);
    while grid.is_valid_point(&p) {
        while grid.is_valid_point(&p) {
            for dir in
                [LEFT, RIGHT, TOP, BOTTOM, TOP | LEFT, TOP | RIGHT, BOTTOM | LEFT, BOTTOM | RIGHT]
            {
                if grid.match_at_point_in_direction(&input, &p, dir) {
                    ret += 1
                }
            }
            p.move_next(RIGHT);
        }

        p.x = 0;
        p.move_next(BOTTOM);
    }

    ret
}

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid = Grid::new(&content);

    let result = count_all_occurences(&grid, "XMAS");

    Ok(result)
}
