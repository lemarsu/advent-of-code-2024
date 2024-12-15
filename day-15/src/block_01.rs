use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Empty,
    Box,
    Wall,
    Robot,
}

impl FromStr for Block {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('#') => Ok(Self::Wall),
            Some('@') => Ok(Self::Robot),
            Some('.') => Ok(Self::Empty),
            Some('O') => Ok(Self::Box),
            None => {
                println!("from_str(None): {}", s);
                Ok(Self::Empty)
            },
            _ => unreachable!(),
        }
    }
}
