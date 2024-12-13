use std::{error::Error, fs};

use crate::{utils::calc_prize, ArcadeConfig, Point};

pub fn main(file: &str) -> Result<i64, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let configs: Vec<_> = ArcadeConfig::parse(&content)
        .into_iter()
        .map(|config| {
            let mut c = config.clone();
            c.prize += 10000000000000.into();
            c
        })
        .collect();

    let mut solutions = Vec::new();
    for config in configs.iter() {
        let mut matrix = config.build_matrix();
        let fact0 = matrix[0][0];
        let fact1 = matrix[1][0];
        for i in 0..3 {
            matrix[1][i] *= fact0;
            matrix[0][i] *= fact1;
        }

        for i in 0..3 {
            matrix[1][i] -= matrix[0][i];
        }
        if matrix[1][2] % matrix[1][1] == 0 {
            let res_y = matrix[1][2] / matrix[1][1];
            let step_x = matrix[0][2] - matrix[0][1] * res_y;
            if step_x % matrix[0][0] == 0 {
                let res_x = step_x / matrix[0][0];
                solutions.push(Point::new(res_x, res_y));
            }
        }
    }

    let sum = solutions.into_iter().fold(0, |acc, p| acc + calc_prize(p));

    Ok(sum)
}
