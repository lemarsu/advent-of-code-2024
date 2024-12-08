use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Grid {
    content: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(str: &str) -> Self {
        let content = str
            .split("\n")
            .map(|line| line.chars().collect())
            .filter(|chars: &Vec<char>| !chars.is_empty())
            .collect();

        Self { content }
    }

    pub fn get_at(&self, pos: &Point) -> char {
        self.get(pos.x, pos.y)
    }

    fn get(&self, x: i32, y: i32) -> char {
        self.content[y as usize][x as usize]
    }

    fn rows(&self) -> usize {
        self.content.len()
    }

    fn cols(&self) -> usize {
        if self.rows() > 0 {
            self.content[0].len()
        } else {
            0
        }
    }

    pub fn is_valid_point(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols() as i32 && point.y >= 0 && point.y < self.rows() as i32
    }

    pub fn each_char_pos(&self) -> GridCharPosIterator {
        GridCharPosIterator::new(self.clone())
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
