use crate::Point;

pub fn calc_prize(result: Point) -> i64 {
    3 * result.x as i64 + result.y as i64
}
