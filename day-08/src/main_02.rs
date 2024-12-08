use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

use crate::{couples::CouplesIterator, grid::Grid, point::Point};

pub fn walk_on(grid: &Grid, p: &Point, dir: &Point, steps: &mut HashSet<Point>) {
    let mut p = p.clone();

    while grid.is_valid_point(&p) {
        steps.insert(p);
        p += *dir;
    }
}

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid = Grid::new(&content);

    let mut antennas = HashMap::new();
    for (char, pos) in grid.each_char_pos().filter(|(char, _)| char != &'.') {
        if !antennas.contains_key(&char) {
            antennas.insert(char, Vec::new());
        }
        let positions = antennas.get_mut(&char).unwrap();
        positions.push(pos);
    }

    let mut antinodes = HashSet::new();

    for char in antennas.keys() {
        let iterator = CouplesIterator::new(&antennas[char]);
        for (a, b) in iterator {
            let direction = a - b;
            walk_on(&grid, &a, &direction, &mut antinodes);
            walk_on(&grid, &b, &-direction, &mut antinodes);
        }
    }

    Ok(antinodes.len() as i32)
}
