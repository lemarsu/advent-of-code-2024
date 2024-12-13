mod cli;
mod arcade;
mod main_01;
mod main_02;
mod matrix;
mod point;
mod utils;

pub use arcade::ArcadeConfig;
pub use point::Point;
pub use matrix::Matrix;

use crate::cli::Cli;
use clap::Parser as ClapParser;

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        main_01::main(&args.file).unwrap()
    } else {
        main_02::main(&args.file).unwrap()
    };

    println!("Result: {}", result);
}
