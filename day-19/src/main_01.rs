use std::{error::Error, fs};

use crate::{towel::can_make_onsen_flag, TowelInfos};

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let infos: TowelInfos = content.parse().unwrap();

    let sum = infos.desired.into_iter().fold(0, |acc, word| {
        if can_make_onsen_flag(&word, &infos.available) {
            acc + 1
        } else {
            acc
        }
    });

    Ok(sum)
}
