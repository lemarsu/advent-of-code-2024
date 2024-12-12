use std::{env, error::Error, fs};

use crate::{grid::Grid, region::Region};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let grid = Grid::new(&content);
    let regions = Region::discover_all_region(&grid);

    let sum = regions.iter().fold(0, |acc, region| {
        let fences = region.count_fences(&grid);
        if let Ok(value) = env::var("DEBUG") {
            if value == "1" {
                println!("Region {}: {} fences", region.plant, fences);
            }
        }
        acc + region.area() * fences
    });

    Ok(sum)
}
