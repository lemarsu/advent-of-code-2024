use std::{collections::HashMap, error::Error, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Stone {
    pub value: usize,
    pub index: usize,
}

impl Stone {
    pub fn new(value: usize) -> Self {
        Self::with_index(value, 0)
    }

    pub fn with_index(value: usize, index: usize) -> Self {
        Self { value, index }
    }

    pub fn next_with_value(&self, value: usize) -> Self {
        Self::with_index(value, self.index + 1)
    }

    pub fn blink_once(&self) -> Vec<Stone> {
        if self.value == 0 {
            vec![self.next_with_value(1)]
        } else if self.value.ilog10() % 2 == 1 {
            num_split(self.value).iter().map(|i| self.next_with_value(*i)).collect()
        } else {
            vec![self.next_with_value(self.value * 2024)]
        }
    }
}

pub fn num_split(num: usize) -> Vec<usize> {
    let log10 = num.ilog10();
    let pow = 10_usize.pow(log10 / 2 + 1);
    vec![num / pow, num % pow]
}

pub fn count_stone(cache: &mut HashMap<Stone, usize>, stone: &Stone, steps: usize) -> usize {
    if let Some(value) = cache.get(stone) {
        return *value;
    }
    if steps == 0 {
        return 0;
    }
    let next_stones = stone.blink_once();
    let add = if next_stones.len() > 1 { 1 } else { 0 };
    let ret = next_stones.iter().fold(add, |acc, s| acc + count_stone(cache, s, steps - 1));
    cache.insert(stone.clone(), ret);
    ret
}

pub fn main(file: &str) -> Result<usize, Box<dyn Error>> {
    let mut content = fs::read_to_string(file)?;

    // remove EOL
    content.remove(content.len() - 1);

    let stones: Vec<usize> = content.split(" ").map(|part| part.parse().unwrap()).collect();

    let mut cache = HashMap::new();
    let total = stones.len()
        + stones
            .iter()
            .map(|s| Stone::new(*s))
            .fold(0, |acc, stone| acc + count_stone(&mut cache, &stone, 75));

    Ok(total)
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
