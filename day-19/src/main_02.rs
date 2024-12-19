use std::{collections::HashMap, error::Error, fs};

use crate::{towel::all_onsen_flag, TowelInfos};

fn progress(str: &str) {
    use std::io::{stdout, Write};
    print!("\r{}", str);
    stdout().flush().unwrap();
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let infos: TowelInfos = content.parse().unwrap();

    let max = infos.desired.len();
    let sum = infos
        .desired
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, word)| {
            progress(&format!("trying ({}/{}) {}", i + 1, max, word.iter().collect::<String>()));
            let mut cache: HashMap<(Vec<char>, Vec<Vec<char>>), usize> = HashMap::new();
            acc + all_onsen_flag(&word, &infos.available, &mut cache)
        });
    println!("");

    Ok(sum)
}
