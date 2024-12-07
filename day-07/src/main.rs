mod cli;

mod apply_op;
mod calculus;
mod combination;
mod main_01;
mod main_02;

use crate::cli::Cli;
use apply_op::ApplyOp;
use calculus::Calculus;
use clap::Parser as ClapParser;
use std::{error::Error, fs};

fn step_main<T: ApplyOp>(file: &str) -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let operations: Vec<Calculus> =
        content.split("\n").filter(|line| *line != "").map(|line| line.parse().unwrap()).collect();

    let total = operations.len();
    let count = operations
        .iter()
        .enumerate()
        .filter(|(i, operation)| {
            use std::io::{stdout, Write};
            print!("\rOp: {}/{} ({:.2} %)", i + 1, total, ((i + 1) * 100) as f32 / (total as f32));
            stdout().flush().unwrap();
            operation.is_valid::<T>()
        })
        .map(|(_, op)| op.result)
        .sum();

    println!("");
    Ok(count)
}

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        step_main::<main_01::Operator>(&args.file).unwrap()
    } else {
        step_main::<main_02::Operator>(&args.file).unwrap()
    };

    println!("Result: {}", result);
}
