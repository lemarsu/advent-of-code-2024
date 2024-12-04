use std::{error::Error, fs};

use crate::{
    direction::{BOTTOM, LEFT, RIGHT, TOP},
    grid::Grid,
    point::Point,
};

fn count_all_crosses(grid: &Grid, input: &str) -> i32 {
    let input: Vec<_> = input.chars().collect();

    let mut ret = 0;
    let mut p = Point::new(0, 0);
    while grid.is_valid_point(&p) {
        while grid.is_valid_point(&p) {
            let tl = grid.match_at_point_in_direction(&input, &p.next(TOP | LEFT), BOTTOM | RIGHT);
            let tr = grid.match_at_point_in_direction(&input, &p.next(TOP | RIGHT), BOTTOM | LEFT);
            let bl = grid.match_at_point_in_direction(&input, &p.next(BOTTOM | LEFT), TOP | RIGHT);
            let br = grid.match_at_point_in_direction(&input, &p.next(BOTTOM | RIGHT), TOP | LEFT);
            ret += i32::from(tl && tr)
                + i32::from(tr && br)
                + i32::from(br && bl)
                + i32::from(bl && tl);
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
    let result = count_all_crosses(&grid, "MAS");

    Ok(result)
}
