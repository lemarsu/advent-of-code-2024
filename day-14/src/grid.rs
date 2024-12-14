use crate::{Direction, Point, Robot};

#[derive(Debug, Clone)]
pub struct Grid {
    content: Vec<Vec<Vec<Robot>>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn empty(cols: usize, rows: usize) -> Self {
        let mut content = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(Vec::<Robot>::new());
            }
            content.push(row);
        }

        Self { rows, cols, content }
    }

    pub fn set_at(&mut self, pos: &Point) -> &mut Vec<Robot> {
        self.set(pos.x, pos.y)
    }

    fn set(&mut self, x: i32, y: i32) -> &mut Vec<Robot> {
        &mut self.content[y as usize][x as usize]
    }


    pub fn get_at(&self, pos: &Point) -> Vec<Robot> {
        self.get(pos.x, pos.y)
    }

    fn get(&self, x: i32, y: i32) -> Vec<Robot> {
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
    type Item = (Vec<Robot>, Point);

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
