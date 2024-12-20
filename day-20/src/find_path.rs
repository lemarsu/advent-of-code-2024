use std::collections::{BinaryHeap, HashSet};

use crate::{Block, Path, Grid, Point};

pub fn find_path(grid: &Grid<Block>, start: &Point, stop: &Point) -> Option<Path> {
    let mut visited = HashSet::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(Path::new(start, stop));

    while let Some(path) = frontier.pop() {
        if path.current == *stop {
            return Some(path);
        }
        let paths = path.next(grid);
        for path in paths.into_iter() {
            frontier.push(path);
        }
        visited.insert(path.current);
    }

    None
}
