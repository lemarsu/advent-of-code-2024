use std::str::FromStr;

pub struct TowelInfos {
    pub available: Vec<Vec<char>>,
    pub desired: Vec<Vec<char>>,
}

impl FromStr for TowelInfos {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("\n\n").collect();

        let mut available: Vec<Vec<_>> =
            parts[0].split(", ").map(|word| word.chars().collect()).collect();

        let mut desired: Vec<Vec<_>> = parts[1]
            .split("\n")
            .filter(|line| *line != "")
            .map(|word| word.chars().collect())
            .collect();

        available.sort_by_key(|word| -(word.len() as i32));
        desired.sort_by_key(|word| word.len());

        Ok(Self { available, desired })
    }
}
