use std::{error::Error, fs};

use crate::secret::SecretIterator;

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let secrets: Vec<usize> =
        content.split("\n").filter(|line| *line != "").map(|line| line.parse().unwrap()).collect();

    let sum = secrets
        .into_iter()
        .map(|secret| SecretIterator::new(secret).nth(1999).unwrap())
        .fold(0, |acc, secret| acc + secret);

    Ok(sum)
}
