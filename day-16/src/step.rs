use crate::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Forward,
    Turn(Direction),
}

use Step::*;

impl Step {
    pub fn cost(&self) -> usize {
        match self {
            Forward => 1,
            Turn(_) => 1000,
        }
    }
}
