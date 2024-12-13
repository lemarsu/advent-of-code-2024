use std::{error::Error, fs};

use crate::{ArcadeConfig, utils::calc_prize};

pub fn main(file: &str) -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let configs = ArcadeConfig::parse(&content);

    let sum = configs
        .iter()
        .filter_map(|conf| conf.find_solution())
        .fold(0, |acc, solution| acc + calc_prize(solution));

    Ok(sum)
}
