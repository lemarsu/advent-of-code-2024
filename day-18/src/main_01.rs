use std::{error::Error, fs};

use crate::{
    path::{debug_path, find_path},
    Block, Grid, Point,
};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let is_sample = file.ends_with("sample");

    let points: Vec<_> = content
        .split("\n")
        .filter(|line| *line != "")
        .map(|line| {
            let nums: Vec<_> = line.split(",").map(|i| i.parse().unwrap()).collect();
            Point::new(nums[0], nums[1])
        })
        .collect();

    let len = if is_sample { 7 } else { 71 };
    let mut grid: Grid<Block> = Grid::empty(len, len, Block::Empty);

    let start = Point::new(0, 0);
    let stop = Point::new((len - 1) as i32, (len - 1) as i32);
    let slice = if is_sample { &points[0..12] } else { &points[0..1024] };

    for point in slice {
        *grid.set_at(point) = Block::Wall;
    }

    if let Some(path) = find_path(&grid, &start, &stop) {
        debug_path(&grid, &path);

        let sum = path.len() - 1;

        return Ok(sum);
    } else {
        println!("No path...");
    }

    Ok(0)
}
