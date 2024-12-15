use std::{error::Error, fs};

use crate::{block_01::Block, Direction, Grid, Point};

fn find_next_empty(grid: &Grid<Block>, point: &Point, dir: Direction) -> Option<Point> {
    let mut p = point.clone();
    loop {
        p.move_next(dir);
        match grid.get_at(&p) {
            // Looking for next point...
            Block::Box => {},
            // The box chain to a wall, stopping
            Block::Wall => return None,
            // Found empty, move box `next` here.
            Block::Empty => return Some(p),
            Block::Robot => unreachable!("A robot here ? Why !?!"),
        }
    }
}

fn run_movements(start: &Point, grid: &mut Grid<Block>, movements: Vec<Direction>) {
    let mut p = start.clone();

    for dir in movements {
        let next = p.next(dir);
        match grid.get_at(&next) {
            Block::Empty => p.move_next(dir),
            // Nothing to do, movement is blocked
            Block::Wall => {},
            Block::Box => {
                if let Some(n) = find_next_empty(&grid, &next, dir) {
                    *grid.set_at(&next) = Block::Empty;
                    *grid.set_at(&n) = Block::Box;
                    p.move_next(dir);
                }
            },
            Block::Robot => unreachable!("A robot here ? Why !?!"),
        }
    }
}

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let parts: Vec<_> = content.split("\n\n").collect();
    let grid_str = parts[0];
    let movements_str = parts[1];

    let mut grid: Grid<Block> = Grid::new(grid_str);
    let movements: Vec<Direction> = movements_str
        .split("")
        .filter(|s| *s != "" && *s != "\n")
        .map(|s| s.parse().unwrap())
        .collect();

    let start = grid.each_item_pos().find(|(b, _)| *b == Block::Robot).unwrap().1.clone();

    *grid.set_at(&start) = Block::Empty;

    run_movements(&start, &mut grid, movements);

    let sum = grid
        .each_item_pos()
        .filter(|(b, _)| matches!(b, Block::Box))
        .map(|(_, p)| p)
        .fold(0, |acc, p| acc + p.x + p.y * 100);

    Ok(sum)
}
