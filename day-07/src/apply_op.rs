pub trait ApplyOp
where
    Self: Sized,
{
    fn generate_for(size: usize) -> Vec<Vec<Self>>;
    fn apply(&self, a: i64, b: i64) -> i64;
}
