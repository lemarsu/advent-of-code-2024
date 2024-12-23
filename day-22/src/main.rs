mod cli;

mod main_01;
mod main_02;
mod secret;

use crate::cli::Cli;
use clap::Parser as ClapParser;

pub use secret::SecretIterator;

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        main_01::main(&args.file).unwrap()
    } else {
        main_02::main(&args.file).unwrap()
    };

    println!("Result: {}", result);
}