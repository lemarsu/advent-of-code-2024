mod cli;

mod code;
mod costs_calculator;
mod direction;
mod main_01;
mod main_02;
mod pad;
mod point;

use crate::cli::Cli;
use clap::Parser as ClapParser;

pub use code::Code;
pub use costs_calculator::CostCalculator;
pub use direction::Direction;
pub use pad::{Pad, PadType};
pub use point::Point;

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        main_01::main(&args.file).unwrap()
    } else {
        main_02::main(&args.file).unwrap()
    };

    println!("Result: {}", result);
}
