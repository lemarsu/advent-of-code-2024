use std::collections::{BinaryHeap, HashSet};

use crate::{Block, Direction, Grid, Point};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    points: Vec<Point>,
}

impl Path {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add(&self, point: &Point) -> Self {
        let mut path = self.clone();
        path.points.push(point.clone());
        path
    }

    pub fn last(&self) -> &Point {
        &self.points[self.points.len() - 1]
    }

    pub fn points(self) -> Vec<Point> {
        self.points
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.points.len().partial_cmp(&other.points.len()).map(|o| o.reverse())
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.points.len().cmp(&other.points.len()).reverse()
    }
}

pub fn debug_path(grid: &Grid<Block>, path: &Vec<Point>) {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let point = Point::new(x as i32, y as i32);
            let c = if path.contains(&point) {
                '*'
            } else {
                match grid.get_at(&point) {
                    Block::Wall => '#',
                    Block::Empty => '.',
                }
            };
            print!("{}", c);
        }
        println!("");
    }
}

pub fn find_path(grid: &Grid<Block>, start: &Point, stop: &Point) -> Option<Vec<Point>> {
    let mut priority = BinaryHeap::new();
    let mut visited = HashSet::new();

    let path = Path::new().add(&start);
    priority.push(path);

    while let Some(path) = priority.pop() {
        if visited.contains(path.last()) {
            continue;
        }
        if path.last() == stop {
            return Some(path.points());
        }
        let next_points: Vec<_> = Direction::all()
            .iter()
            .map(|dir| path.last().next(*dir))
            .filter(|p| {
                grid.is_valid_point(p) && grid.get_at(p) != Block::Wall && !path.contains(p)
            })
            .collect();

        for point in next_points {
            priority.push(path.add(&point));
        }
        visited.insert(path.last().clone());
    }
    None
}
