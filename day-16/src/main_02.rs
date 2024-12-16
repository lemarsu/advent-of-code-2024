use std::{collections::HashSet, error::Error, fs};

use crate::{
    find_path::find_paths,
    Block::{self, *},
    Direction::*,
    Grid, Raindeer,
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

    let min_cost = paths.iter().fold(usize::MAX, |acc, path| {
        let cost = path.cost();
        if cost < acc {
            cost
        } else {
            acc
        }
    });

    // println!("paths count: {}", paths.len());

    let points: HashSet<_> = paths
        .into_iter()
        .filter(|path| path.cost() == min_cost)
        .flat_map(|path| {
            // println!("path cost: {}", path.cost());
            path.points
        })
        .collect();
    let sum = points.len();

    Ok(sum)
}
