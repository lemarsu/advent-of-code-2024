use std::{collections::HashMap, error::Error, fs};

use crate::{
    find_path::find_path,
    Block::{self, *},
    Direction, Grid, Jump,
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

    let path = find_path(&grid, &start, &stop).unwrap();

    let mut jumps = Vec::new();

    for point in path.path.iter() {
        Direction::all()
            .into_iter()
            .filter_map(|dir| {
                let next = point.next(dir);
                if !grid.is_valid_point(&next) {
                    return None;
                }
                let next2 = next.next(dir);
                if !grid.is_valid_point(&next2) {
                    return None;
                }
                if grid.get_at(&next) == Block::Wall && grid.get_at(&next2) == Block::Empty {
                    Some(Jump::new(point.clone(), next2))
                } else {
                    None
                }
            })
            .for_each(|wall| {
                jumps.push(wall);
            });
    }

    println!("Got {} walls to test", jumps.len());

    let mut jumped_paths = HashMap::new();

    for jump in jumps.into_iter() {
        let from_pos = path.path.iter().position(|p| *p == jump.from).unwrap();
        if let Some(to_pos) = path.path.iter().position(|p| *p == jump.to) {
            let diff = to_pos.saturating_sub(from_pos).saturating_sub(jump.len());
            if diff > 0 {
                let entry = jumped_paths.entry(diff).or_insert(0);
                *entry += 1;
            }
        }
    }

    for (diff, count) in jumped_paths.iter() {
        println!("Found {} paths cost difference of {}", count, diff);
    }

    let sum = jumped_paths.into_iter().fold(
        0,
        |acc, (diff, count)| {
            if diff > 100 {
                acc + count
            } else {
                acc
            }
        },
    );

    Ok(sum)
}
