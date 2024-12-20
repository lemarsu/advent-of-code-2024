use std::ops::Deref;

use crate::{Block, Direction, Grid, Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub from: Point,
    pub to: Point,
    pub current: Point,
    pub path: Vec<Point>,
    pub path_cost: usize,
    pub end_dist_cost: usize,
}

impl Path {
    pub fn new(from: &Point, to: &Point) -> Self {
        let from = from.clone();
        let to = to.clone();
        Self {
            current: from.clone(),
            path: vec![from.clone()],
            end_dist_cost: from.manathan_distance(&to),
            path_cost: 0,
            from,
            to,
        }
    }

    pub fn next(&self, grid: &Grid<Block>) -> Vec<Path> {
        Direction::all()
            .into_iter()
            .filter_map(|dir| {
                let point = self.current.next(dir);
                if grid.is_valid_point(&point)
                    && !self.path.contains(&point)
                    && grid.get_at(&point) == Block::Empty
                {
                    Some(point)
                } else {
                    None
                }
            })
            .map(|p| self.extend(&p))
            .collect()
    }

    pub fn extend(&self, point: &Point) -> Path {
        let mut path = self.clone();
        path.path.push(point.clone());
        path.path_cost += 1;
        path.current = point.clone();
        path.end_dist_cost = point.manathan_distance(&path.to);
        path
    }

    pub fn cost(&self) -> usize {
        self.path_cost + self.end_dist_cost
    }
}

impl Deref for Path {
    type Target = Point;
    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost().partial_cmp(&other.cost()).map(|ord| ord.reverse())
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost()).reverse()
    }
}
