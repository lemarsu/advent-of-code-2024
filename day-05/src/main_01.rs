use std::{error::Error, fs};

use crate::print::Print;

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file).unwrap();

    let print: Print = content.parse().unwrap();
    let lists = print.get_valid_lists();

    Ok(lists.iter().fold(0, |acc, l| acc + l.middle()))
}
