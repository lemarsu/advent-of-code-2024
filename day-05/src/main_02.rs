use std::{error::Error, fs};

use crate::print::Print;

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file).unwrap();

    let print: Print = content.parse().unwrap();
    let mut lists = print.get_invalid_lists();
    lists.iter_mut().for_each(|list| list.sort_with_rules(&print.rules));

    Ok(lists.iter().fold(0, |acc, l| acc + l.middle()))
}
