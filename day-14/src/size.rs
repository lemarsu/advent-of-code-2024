use crate::Point;

pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn quadrant(&self, point: &Point) -> Option<usize> {
        let half_width = (self.width / 2) as i32;
        let half_height = (self.height / 2) as i32;
        if point.x == half_width || point.y == half_height {
            return None
        }

        let mut quadrant = 0;
        if point.x > half_width {
            quadrant += 2;
        }
        if point.y > half_height {
            quadrant += 1;
        }
        Some(quadrant)
    }
}
