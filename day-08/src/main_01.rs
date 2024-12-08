use std::{collections::{HashMap, HashSet}, error::Error, fs};

use crate::{couples::CouplesIterator, grid::Grid};

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
            let dist = a - b;
            let antinode_a = a + dist;
            let antinode_b = b - dist;
            if grid.is_valid_point(&antinode_a) {
                antinodes.insert(antinode_a);
            }
            if grid.is_valid_point(&antinode_b) {
                antinodes.insert(antinode_b);
            }
        }
    }

    Ok(antinodes.len() as i32)
}
