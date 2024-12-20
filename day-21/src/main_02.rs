use std::{error::Error, fs};

use crate::{Code, CostCalculator};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let codes: Vec<Code> =
        content.split("\n").filter(|line| *line != "").map(|line| line.parse().unwrap()).collect();

    let costs_calculator = CostCalculator::new(25);

    let sum = codes.into_iter().fold(0, |acc, code| acc + costs_calculator.calc(&code));

    Ok(sum)
}
