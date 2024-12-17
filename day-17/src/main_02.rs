use std::{error::Error, fs};

use crate::{Computer, Instruction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Match {
    None,
    Partial(usize),
    Full,
}

fn match_end<T: PartialEq + Eq>(short: &Vec<T>, long: &Vec<T>) -> bool {
    let mut matched = true;
    for i in 0..short.len() {
        if short[short.len() - 1 - i] != long[long.len() - i - 1] {
            matched = false;
            break;
        }
    }
    matched
}

fn match_quine(computer: &Computer, raw: &Vec<u64>, value: u64) -> Match {
    let instructions = Instruction::from_numbers(raw.clone());
    let mut cmp = computer.clone();
    cmp.reg_a = value;
    let out = cmp.run(&instructions);
    if out == *raw {
        Match::Full
    } else if match_end(&raw, &out) {
        Match::Partial(out.len())
    } else {
        Match::None
    }
}

fn find_next(computer: &Computer, raw: &Vec<u64>, prev: Vec<u64>) -> Option<u64> {
    if prev.len() > raw.len() {
        return None;
    }
    let offset = prev.iter().fold(0, |acc, i| (acc << 3) | i);
    for i in 0..8 {
        println!("depth: {}, i: {}, prev: {:?}", prev.len(), i, prev);
        let reg_a = (offset << 3) | i;
        match match_quine(computer, raw, reg_a) {
            Match::None => {},
            Match::Partial(_) => {
                let mut current = prev.clone();
                current.push(i);
                if let Some(next) = find_next(computer, raw, current) {
                    return Some(next);
                }
            },
            Match::Full => {
                return Some(reg_a);
            },
        }
    }
    println!("prev: {:?}", prev);
    None
}

pub fn main(file: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let (computer, raw) = Computer::parse(&content);

    let reg_a = find_next(&computer, &raw, Vec::new());

    Ok(vec![reg_a.unwrap()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_end_test() {
        assert!(match_end(&vec![1, 2], &vec![1, 2]));
        assert!(match_end(&vec![1, 2], &vec![3, 1, 2]));
        assert!(match_end(&vec![1, 2], &vec![4, 3, 1, 2]));
        assert!(!match_end(&vec![1, 2], &vec![3, 1, 3]));
        assert!(!match_end(&vec![1, 2], &vec![3, 3, 2]));
    }
}
