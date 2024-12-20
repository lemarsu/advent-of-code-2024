use crate::Point;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Jump {
    pub from: Point,
    pub to: Point,
}

impl Jump {
    pub fn new(from: Point, to: Point) -> Self {
        Self { from, to }
    }

    pub fn len(&self) -> usize {
        self.from.manathan_distance(&self.to)
    }
}
