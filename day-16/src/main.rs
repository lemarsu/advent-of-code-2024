mod cli;

mod block;
mod direction;
mod find_path;
mod grid;
mod main_01;
mod main_02;
mod path;
mod point;
mod raindeer;
mod step;

use crate::cli::Cli;
use clap::Parser as ClapParser;

pub use block::Block;
pub use direction::Direction;
pub use grid::Grid;
pub use path::Path;
pub use point::Point;
pub use raindeer::Raindeer;
pub use step::Step;

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        main_01::main(&args.file).unwrap()
    } else {
        main_02::main(&args.file).unwrap()
    };

    println!("Result: {}", result);
}
