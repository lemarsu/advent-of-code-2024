use regex::Regex;

use crate::{Matrix, Point};

#[derive(Debug, Clone)]
pub struct ArcadeConfig {
    pub button_a: Point,
    pub button_b: Point,
    pub prize: Point,
}

impl ArcadeConfig {
    pub fn new(button_a: Point, button_b: Point, prize: Point) -> Self {
        Self { button_a, button_b, prize }
    }

    pub fn find_solution(&self) -> Option<Point> {
        for a in 0..100 {
            for b in 0..100 {
                if self.button_a * a.into() + self.button_b * b.into() == self.prize {
                    return Some(Point::new(a, b));
                }
            }
        }
        None
    }

    pub fn build_matrix(&self) -> Matrix<3, 2, i64> {
        let mut matrix = Matrix::new(0);
        matrix[0][0] = self.button_a.x;
        matrix[0][1] = self.button_b.x;
        matrix[0][2] = self.prize.x;
        matrix[1][0] = self.button_a.y;
        matrix[1][1] = self.button_b.y;
        matrix[1][2] = self.prize.y;
        matrix
    }

    pub fn parse(str: &str) -> Vec<ArcadeConfig> {
        let re_button = Regex::new(r"Button ([AB]): X\+(\d+), Y\+(\d+)").unwrap();
        let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        str.split("\n\n")
            .map(|str| {
                let strings: Vec<_> = str.split("\n").collect();
                let [_, x, y] = re_button.captures(&strings[0]).unwrap().extract().1;
                let button_a = Point::new(x.parse().unwrap(), y.parse().unwrap());

                let [_, x, y] = re_button.captures(&strings[1]).unwrap().extract().1;
                let button_b = Point::new(x.parse().unwrap(), y.parse().unwrap());

                let [x, y] = re_prize.captures(&strings[2]).unwrap().extract().1;
                let prize = Point::new(x.parse().unwrap(), y.parse().unwrap());

                ArcadeConfig::new(button_a, button_b, prize)
            })
            .collect()
    }
}
