mod cli;

mod block_01;
mod block_02;
mod direction;
mod grid;
mod main_01;
mod main_02;
mod point;

use crate::cli::Cli;
use clap::Parser as ClapParser;

pub use direction::Direction;
pub use grid::Grid;
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