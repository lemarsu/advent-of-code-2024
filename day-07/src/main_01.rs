use crate::{apply_op::ApplyOp, combination};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mul,
}

impl ApplyOp for Operator {
    fn generate_for(size: usize) -> Vec<Vec<Self>> {
        use Operator::*;

        combination::each(&vec![Add, Mul], size - 1).into_iter().map(|ops| ops).collect()
    }

    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
        }
    }
}