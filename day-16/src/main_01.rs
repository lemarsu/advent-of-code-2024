use std::{error::Error, fs};

use crate::{
    find_path::find_paths,
    Block::{self, *},
    Direction::*,
    Grid, Raindeer, Step,
};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let mut grid: Grid<Block> = Grid::new(&content);

    let start = grid
        .each_item_pos()
        .find_map(|(b, p)| {
            if b == Start {
                return Some(p);
            }
            None
        })
        .unwrap();

    let stop = grid
        .each_item_pos()
        .find_map(|(b, p)| {
            if b == End {
                return Some(p);
            }
            None
        })
        .unwrap();

    *grid.set_at(&start) = Empty;
    *grid.set_at(&stop) = Empty;

    let raindeer = Raindeer::new(start, Left);
    let paths = find_paths(&grid, &raindeer, &stop);

    println!("paths count: {}", paths.len());
    let path = &paths[0];
    println!("Deer: {:?}", raindeer);
    println!("Stop: {:?}", stop);
    println!(
        "Path size: {:?}",
        path.clone().steps.iter().filter(|s| matches!(s, Step::Forward)).count()
    );

    let sum = path.steps.iter().fold(0, |acc, step| acc + step.cost());

    Ok(sum)
}
