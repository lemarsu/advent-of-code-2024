use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

use crate::secret::SecretIterator;

#[inline]
fn progress(str: &str) {
    use std::io::{stdout, Write};
    print!("\r{}", str);
    stdout().flush().unwrap();
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let secrets: Vec<usize> =
        content.split("\n").filter(|line| *line != "").map(|line| line.parse().unwrap()).collect();

    let mut best_bananas = 0;

    let mut secret_changes = HashMap::new();
    let mut all_changes = HashSet::new();
    let secrets_len = secrets.len();
    for (i, secret) in secrets.iter().enumerate() {
        progress(&format!("Caching changes for secret {} of {}", i + 1, secrets_len));
        let iter = SecretIterator::new(*secret);
        secret_changes.entry(secret).or_insert(HashMap::new());
        for win in iter.take(2000).collect::<Vec<_>>().windows(5) {
            let price = win[win.len() - 1] % 10;
            let current_change: Vec<_> = win
                .windows(2)
                .map(|couple| (couple[1] as i64 % 10) - (couple[0] as i64 % 10))
                .collect();
            all_changes.insert(current_change.clone());
            secret_changes.get_mut(&secret).unwrap().entry(current_change).or_insert(price);
        }
    }
    println!("");

    let mut all_changes: Vec<_> = all_changes.iter().collect();
    all_changes.sort();

    for change in all_changes.into_iter() {
        progress(&format!("Trying change [{}, {}, {}, {}]    ", change[0], change[1], change[2], change[3]));
        let bananas = secrets
            .iter()
            .map(|secret| &secret_changes[secret])
            .fold(0, |acc, hash| acc + hash.get(change).unwrap_or(&0));
        if bananas > best_bananas {
            best_bananas = bananas;
            println!("\rnew best: {}!                    ", best_bananas);
        }
    }
    println!("");

    Ok(best_bananas)
}
