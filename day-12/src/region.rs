use std::collections::{HashSet, VecDeque};

use crate::{Direction::*, FenceWalker, Grid, Point, PointBorder, Rotation};

#[derive(Debug, Clone)]
pub struct Region {
    pub plant: char,
    pub points: Vec<Point>,
    pub perimeter: usize,
}

impl Region {
    pub fn discover_all_region(grid: &Grid) -> Vec<Self> {
        let mut remaining_points: HashSet<_> = grid.each_char_pos().map(|(_, pos)| pos).collect();
        let mut regions = Vec::new();

        while !remaining_points.is_empty() {
            let point = remaining_points.iter().nth(0).unwrap().clone();
            remaining_points.remove(&point);
            let region = Region::discover_region(&grid, &point, &mut remaining_points);
            regions.push(region);
        }

        regions
    }

    pub fn discover_region(
        grid: &Grid,
        point: &Point,
        remaining_points: &mut HashSet<Point>,
    ) -> Self {
        let plant = grid.get_at(point);
        let mut points = HashSet::new();
        let mut perimeter = 0;
        let mut to_visit = VecDeque::new();
        to_visit.push_front(point.clone());

        while !to_visit.is_empty() {
            let point = to_visit.pop_front().unwrap();
            remaining_points.remove(&point);
            points.insert(point);
            let cardinal_points = [Left, Right, Top, Bottom].iter().map(|dir| point.next(*dir));
            for cardinal_point in cardinal_points {
                if points.contains(&cardinal_point) {
                    continue;
                }
                if !grid.is_valid_point(&cardinal_point) || grid.get_at(&cardinal_point) != plant {
                    perimeter += 1;
                } else {
                    if !to_visit.contains(&cardinal_point) {
                        to_visit.push_front(cardinal_point);
                    }
                }
            }
        }

        Self { plant, perimeter, points: points.into_iter().collect() }
    }

    pub fn area(&self) -> usize {
        self.points.len()
    }

    pub fn cost(&self) -> usize {
        self.perimeter * self.area()
    }

    pub fn count_fences(&self, grid: &Grid) -> usize {
        let mut remaining_points = self.points.clone();
        let mut fences = 0;
        let mut full_trail = HashSet::new();
        while !remaining_points.is_empty() {
            let point = remaining_points.pop().unwrap();
            let borders: Vec<_> = grid
                .get_borders(&point)
                .into_iter()
                .filter(|border| !full_trail.contains(&PointBorder::new(point, *border)))
                .collect();
            if borders.len() > 0 {
                let direction = borders[0].turn(Rotation::Left);
                let walker = FenceWalker::new(grid, Rotation::Right, &point, direction);
                let (count, trail) = walker.walk();
                fences += count;
                for i in trail.into_iter() {
                    full_trail.insert(i);
                }
            }
        }
        fences
    }
}
