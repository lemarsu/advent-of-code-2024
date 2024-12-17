mod cli;

mod computer;
mod instruction;
mod main_01;
mod main_02;

pub use computer::Computer;
pub use instruction::Instruction;

use crate::cli::Cli;
use clap::Parser as ClapParser;

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        main_01::main(&args.file).unwrap()
    } else {
        main_02::main(&args.file).unwrap()
    };

    println!(
        "Result: {}",
        result.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")
    );
}
