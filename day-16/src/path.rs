use std::collections::HashSet;

use crate::{Direction::*, Point, Raindeer, Step};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub steps: Vec<Step>,
    pub start: Raindeer,
    pub stop: Raindeer,
    pub points: Vec<Point>,
    past: HashSet<Raindeer>,
}

impl Path {
    fn new(start: Raindeer, stop: Raindeer, steps: Vec<Step>) -> Self {
        Self { steps, start, stop, points: Vec::new(), past: HashSet::new() }
    }

    pub fn empty(deer: &Raindeer) -> Self {
        Self::new(deer.clone(), deer.clone(), Vec::new())
    }

    pub fn cost(&self) -> usize {
        self.steps.iter().fold(0, |acc, step| acc + step.cost())
    }

    pub fn add_step(&self, step: Step) -> Option<Self> {
        let mut path = self.clone();
        match step {
            Step::Forward => {
                path.stop.move_next();
            },
            Step::Turn(Left) => {
                path.stop.direction = path.stop.direction.left();
            },
            Step::Turn(Right) => {
                path.stop.direction = path.stop.direction.right();
            },
            Step::Turn(dir) => {
                unreachable!("Step turn can have {:?} has value", dir);
            },
        }
        if path.past.contains(&path.stop) {
            return None;
        }
        path.steps.push(step);
        path.past.insert(path.stop.clone());
        path.points.push(path.stop.pos.clone());
        Some(path)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost().cmp(&other.cost()).reverse())
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
