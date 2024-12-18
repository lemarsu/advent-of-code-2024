use std::{error::Error, fs};

use crate::{
    path::{debug_path, find_path},
    Block, Grid, Point,
};

fn progress(str: &str) {
    use std::io::{stdout, Write};
    print!("\r{}", str);
    stdout().flush().unwrap();
}

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
    let max_step = if is_sample { 12 } else { 1024 };
    let slice = &points[0..max_step];

    for point in slice {
        *grid.set_at(point) = Block::Wall;
    }

    let mut old_path = Vec::new();
    let mut step = max_step - 1;
    while let Some(path) = find_path(&grid, &start, &stop) {
        step += 1;
        progress(&format!("step: {}", step));
        *grid.set_at(&points[step]) = Block::Wall;
        old_path = path.clone();
    }

    println!("\nLast path:");
    debug_path(&grid, &old_path);
    println!("Step is: {}. Coords are: {:?}", step, points[step]);

    Ok(0)
}
