mod cli;

mod direction;
mod fence_walker;
mod grid;
mod main_01;
mod main_02;
mod point;
mod point_border;
mod region;
mod rotation;

use crate::cli::Cli;
use clap::Parser as ClapParser;

pub use direction::Direction;
pub use fence_walker::FenceWalker;
pub use grid::Grid;
pub use point::Point;
pub use point_border::PointBorder;
pub use region::Region;
pub use rotation::Rotation;

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        main_01::main(&args.file).unwrap()
    } else {
        main_02::main(&args.file).unwrap()
    };

    println!("Result: {}", result);
}
