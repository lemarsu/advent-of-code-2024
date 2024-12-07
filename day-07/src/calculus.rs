use std::str::FromStr;

use crate::apply_op::ApplyOp;

pub struct Calculus {
    pub result: i64,
    pub operands: Vec<i64>,
}

impl Calculus {
    pub fn is_valid<T: ApplyOp>(&self) -> bool {
        let operators_list = T::generate_for(self.operands.len());
        operators_list.iter().any(|ops| {
            let head = self.operands[0];
            let tail = &self.operands[1..];
            let result = ops.iter().zip(tail).fold(head, |acc, (op, num)| op.apply(acc, *num));
            result == self.result
        })
    }
}

impl FromStr for Calculus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split(": ").collect();
        let result = splits[0].parse().unwrap();
        let operands = splits[1].split(" ").map(|op| op.parse().unwrap()).collect();
        Ok(Self { result, operands })
    }
}
