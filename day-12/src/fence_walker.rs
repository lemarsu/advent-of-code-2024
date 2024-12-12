use crate::{Direction, Grid, Point, PointBorder, Rotation};

pub struct FenceWalker<'a> {
    grid: &'a Grid,
    rotation: Rotation,
    point: &'a Point,
    direction: Direction,
}

impl<'a> FenceWalker<'a> {
    pub fn new(grid: &'a Grid, rotation: Rotation, point: &'a Point, direction: Direction) -> Self {
        Self { grid, rotation, point, direction }
    }

    pub fn walk(&self) -> (usize, Vec<PointBorder>) {
        let mut p = self.point.clone();
        let mut dir = self.direction;
        // We are at a fence, but we will loop back on this fence
        let mut fences = 0;
        let mut trail = Vec::new();

        loop {
            // self.rotation is assumed left for the comments
            // If we walk following dir, the wall is on the left
            trail.push(PointBorder::new(p.clone(), dir.turn(self.rotation)));
            // Border in front so turn right
            if self.grid.has_border_in_direction(&p, dir) {
                dir = dir.turn(self.rotation.opposite());
                fences += 1;
            } else {
                p.move_next(dir);
                // There is no border on left, so turn left and
                // move up to get the fence on the left
                let next_dir = dir.turn(self.rotation);
                if !self.grid.has_border_in_direction(&p, next_dir) {
                    dir = next_dir;
                    p.move_next(dir);
                    fences += 1;
                }
            }
            if p == *self.point && dir == self.direction {
                break;
            }
        }

        (fences, trail)
    }
}
