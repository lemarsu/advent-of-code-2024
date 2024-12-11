use std::{error::Error, fs};

pub fn blink_once(stones: &Vec<usize>) -> Vec<usize> {
    stones.iter().flat_map(|stone| {
        if *stone == 0 {
            vec![1]
        } else if stone.ilog10() % 2 == 1 {
            num_split(*stone)
        } else {
            vec![*stone * 2024]
        }
    }).collect()
}

pub fn num_split(num: usize) -> Vec<usize> {
    let log10 = num.ilog10();
    let pow = 10_usize.pow(log10 / 2 + 1);
    vec![num / pow, num % pow]
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let mut content = fs::read_to_string(file)?;

    // remove EOL
    content.remove(content.len() - 1);

    let mut stones: Vec<usize> = content.split(" ").map(|part| part.parse().unwrap()).collect();

    for _ in 0..25 {
        stones = blink_once(&stones);
    }

    Ok(stones.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log10_test() {
        assert_eq!(10_usize.ilog10(), 1);
        assert_eq!(100_usize.ilog10(), 2);
        assert_eq!(150_usize.ilog10(), 2);
        assert_eq!(1000_usize.ilog10(), 3);
    }

    #[test]
    fn num_split_test() {
        assert_eq!(num_split(12), vec![1, 2]);
        assert_eq!(num_split(1234), vec![12, 34]);
        assert_eq!(num_split(123456), vec![123, 456]);
    }
}
