mod cli;

use crate::cli::Cli;
use clap::Parser as ClapParser;
use std::{error::Error, fs, str::FromStr};

#[derive(Debug)]
struct ReportLine(Vec<i32>);

impl FromStr for ReportLine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReportLine(
            s.split(" ").map(|number| number.parse().unwrap()).collect(),
        ))
    }
}

impl ReportLine {
    fn is_valid(&self) -> bool {
        self.0.windows(2).all(|numbers| {
            let diff = i32::abs(numbers[0] - numbers[1]);
            diff >= 1 && diff <= 3
        }) && (self.0.windows(2).all(|n| n[0] > n[1]) || self.0.windows(2).all(|n| n[0] < n[1]))
    }

    fn is_almost_valid(&self) -> bool {
        self.is_valid() || (0..self.0.len()).any(|i| self.without(i).is_valid())
    }

    fn without(&self, idx: usize) -> Self {
        Self(
            self.0
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, v)| *v)
                .collect(),
        )
    }
}

fn read_file(name: &str) -> Result<Vec<ReportLine>, Box<dyn Error>> {
    let content: String = fs::read_to_string(name)?.parse()?;
    let report: Vec<ReportLine> = content
        .split("\n")
        .flat_map(|line| {
            if line == "" {
                return vec![];
            }
            vec![line.parse().unwrap()]
        })
        .collect();
    Ok(report)
}

fn process1(file: &str) -> Result<i32, Box<dyn Error>> {
    let report_lines = read_file(file)?;

    let result = report_lines
        .iter()
        .fold(0, |acc, line| acc + Into::<i32>::into(line.is_valid()));

    Ok(result)
}

fn process2(file: &str) -> Result<i32, Box<dyn Error>> {
    let report_lines = read_file(file)?;

    let result = report_lines.iter().fold(0, |acc, line| {
        acc + Into::<i32>::into(line.is_almost_valid())
    });

    Ok(result)
}

fn main() {
    let args = Cli::parse();

    let result = if args.step == 1 {
        process1(&args.file).unwrap()
    } else {
        process2(&args.file).unwrap()
    };

    println!("Result: {}", result);
}
