use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs,
};

use crate::{direction::Direction::*, grid::Grid, point::Point};

fn score_trail_start(grid: &Grid, s: &Point) -> usize {
    let mut points = VecDeque::new();
    let mut excluded = HashSet::new();
    let mut ret = 0;
    points.push_front(s.clone());
    let directions = [Top, Bottom, Left, Right];
    while !points.is_empty() {
        let p = points.pop_back().unwrap();
        let current_value = grid.get_at(&p);
        if current_value == 9 {
            ret += 1;
        }
        for direction in directions {
            let new_point = p.next(direction);
            if grid.is_valid_point(&new_point)
                && !excluded.contains(&new_point)
                && grid.get_at(&new_point) == current_value + 1
            {
                excluded.insert(new_point);
                points.push_front(new_point);
            }
        }
    }
    ret
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid = Grid::new(&content);

    let starts: Vec<_> =
        grid.each_char_pos().filter(|(height, _)| height == &0).map(|(_, pos)| pos).collect();

    let ret = starts.iter().fold(0, |acc, s| acc + score_trail_start(&grid, &s));

    Ok(ret)
}
