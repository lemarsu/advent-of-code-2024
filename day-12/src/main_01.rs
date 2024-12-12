use std::{error::Error, fs};

use crate::{grid::Grid, region::Region};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid = Grid::new(&content);
    let regions = Region::discover_all_region(&grid);

    let sum = regions.iter().fold(0, |acc, region| acc + region.cost());

    for region in regions.iter() {
        let mut points = region.points.clone();
        points.sort();
    }

    Ok(sum)
}
