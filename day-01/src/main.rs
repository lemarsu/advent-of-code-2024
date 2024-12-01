mod cli;

use crate::cli::Cli;
use clap::Parser as ClapParser;
use std::{error::Error, fs};

#[derive(Debug)]
struct Coordonates(i32, i32);

fn read_file(name: &str) -> Result<Vec<Coordonates>, Box<dyn Error>> {
    let content: String = fs::read_to_string(name)?.parse()?;
    let coords: Vec<Coordonates> = content
        .split("\n")
        .flat_map(|line| {
            if line == "" {
                return vec![];
            }
            let numbers: Vec<i32> = line.split("   ").map(|numbers| return numbers.parse::<i32>().unwrap()).collect();

            return vec![Coordonates(numbers[0], numbers[1])];
        })
        .collect();
    Ok(coords)
}

fn process1(file: &str) -> Result<i32, Box<dyn Error>> {
    let coords = read_file(file)?;

    let mut list1: Vec<i32> = coords.iter().map(|c| c.0).collect();
    let mut list2: Vec<i32> = coords.iter().map(|c| c.1).collect();
    list1.sort();
    list2.sort();

    let result = list1.iter().zip(list2).fold(0, |acc, (i1, i2)| return acc + i32::abs(i1 - i2));

    Ok(result)
}

fn process2(file: &str) -> Result<i32, Box<dyn Error>> {
    let coords = read_file(file)?;

    let result = coords.iter().fold(0, |acc, coord| {
        let same_number_count =
            coords.iter().fold(0, |acc, inner_coord| acc + Into::<i32>::into(inner_coord.1 == coord.0));
        acc + same_number_count * coord.0
    });

    Ok(result)
}

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 { process1(&args.file).unwrap() } else { process2(&args.file).unwrap() };

    println!("Result: {}", result);
}
