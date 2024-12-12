use crate::{Direction, Point};

#[derive(Debug, Clone)]
pub struct Grid {
    content: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn new(str: &str) -> Self {
        let content: Vec<_> = str
            .split("\n")
            .map(|line| line.chars().collect())
            .filter(|chars: &Vec<char>| !chars.is_empty())
            .collect();

        Self { rows: content.len(), cols: content[0].len(), content }
    }

    pub fn get_at(&self, pos: &Point) -> char {
        self.get(pos.x, pos.y)
    }

    fn get(&self, x: i32, y: i32) -> char {
        self.content[y as usize][x as usize]
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    pub fn is_valid_point(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols() as i32 && point.y >= 0 && point.y < self.rows() as i32
    }

    pub fn each_char_pos(&self) -> GridCharPosIterator {
        GridCharPosIterator::new(self.clone())
    }

    pub fn has_border_in_direction(&self, p: &Point, dir: Direction) -> bool {
        let next = p.next(dir);
        !self.is_valid_point(&next) || self.get_at(&p) != self.get_at(&next)
    }

    pub fn get_borders(&self, p: &Point) -> Vec<Direction> {
        Direction::all().into_iter().filter(|dir| self.has_border_in_direction(p, *dir)).collect()
    }
}

pub struct GridCharPosIterator {
    grid: Grid,
    pos: Point,
}

impl GridCharPosIterator {
    pub fn new(grid: Grid) -> Self {
        Self { grid, pos: Point::new(0, 0) }
    }
}

impl Iterator for GridCharPosIterator {
    type Item = (char, Point);

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
