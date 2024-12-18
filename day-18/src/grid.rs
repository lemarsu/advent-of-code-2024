use std::{fmt::Debug, str::FromStr};

use crate::Point;

#[derive(Debug, Clone)]
pub struct Grid<T: Clone + FromStr<Err: Debug>> {
    content: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl<T: Clone + FromStr<Err: Debug>> Grid<T> {
    pub fn empty(x: usize, y: usize, default: T) -> Self {
        let content: Vec<_> = (0..y).map(|_| {
            (0..x).map(|_| default.clone()).collect::<Vec<_>>()
        }).collect();

        Self { rows: content.len(), cols: content[0].len(), content }
    }

    pub fn set_at(&mut self, pos: &Point) -> &mut T {
        self.set(pos.x, pos.y)
    }

    fn set(&mut self, x: i32, y: i32) -> &mut T {
        &mut self.content[y as usize][x as usize]
    }

    pub fn get_at(&self, pos: &Point) -> T {
        self.get(pos.x, pos.y)
    }

    fn get(&self, x: i32, y: i32) -> T {
        self.content[y as usize][x as usize].clone()
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn is_valid_point(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols() as i32 && point.y >= 0 && point.y < self.rows() as i32
    }

    pub fn each_item_pos(&self) -> GridCharPosIterator<T> {
        GridCharPosIterator::new(self.clone())
    }
}

pub struct GridCharPosIterator<T: Clone + FromStr<Err: Debug>> {
    grid: Grid<T>,
    pos: Point,
}

impl<T: Clone + FromStr<Err: Debug>> GridCharPosIterator<T> {
    pub fn new(grid: Grid<T>) -> Self {
        Self { grid, pos: Point::new(0, 0) }
    }
}

impl<T: Clone + FromStr<Err: Debug>> Iterator for GridCharPosIterator<T> {
    type Item = (T, Point);

    fn next(&mut self) -> Option<Self::Item> {
        let current_point = self.pos.clone();
        if self.grid.is_valid_point(&current_point) {
            self.pos.x += 1;
            return Some((self.grid.get_at(&current_point), current_point));
        } else {
            if self.pos.x as usize >= self.grid.cols() {
                self.pos.x = 0;
                self.pos.y += 1;
                return self.next();
            } else {
                None
            }
        }
    }
}
