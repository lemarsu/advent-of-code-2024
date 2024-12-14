mod cli;

mod direction;
mod grid;
mod main_01;
mod main_02;
mod point;
mod robot;
mod size;

use crate::cli::Cli;
use clap::Parser as ClapParser;

pub use direction::Direction;
pub use grid::Grid;
pub use point::Point;
pub use robot::Robot;
pub use size::Size;

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        main_01::main(args.width, args.height, &args.file).unwrap()
    } else {
        main_02::main(args.width, args.height, &args.file).unwrap()
    };

    println!("Result: {}", result);
}
