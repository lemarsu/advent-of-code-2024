use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct Rule(pub i32, pub i32);

impl FromStr for Rule {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s.split("|").map(|num| num.parse().unwrap()).collect();
        if nums.len() != 2 {
            Err("Error".into())
        } else {
            Ok(Self(nums[0], nums[1]))
        }
    }
}
