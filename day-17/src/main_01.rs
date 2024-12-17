use std::{error::Error, fs};

use crate::{Computer, Instruction};

pub fn main(file: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let (mut computer, raw_instructions) = Computer::parse(&content);
    let instructions = Instruction::from_numbers(raw_instructions);

    let output = computer.run(&instructions);

    Ok(output)
}
