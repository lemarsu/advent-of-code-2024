use std::{fmt::{Display, Write}, ops::Deref, str::FromStr};

#[derive(Debug, Clone)]
pub struct Code(Vec<char>);

impl Code {
    pub fn new(chars: Vec<char>) -> Self {
        Self(chars)
    }

    pub fn keys(&self) -> &[char] {
        &self.0
    }

    pub fn numeric_val(&self) -> usize {
        self.0
            .iter()
            .filter(|char| **char >= '0' && **char <= '9')
            .fold(String::new(), |mut acc, c| {
                acc.push(*c);
                acc
            })
            .parse()
            .unwrap()
    }
}

impl Deref for Code {
    type Target = [char];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Code {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Code::new(s.chars().collect()))
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            f.write_char(*c)?;
        }
        Ok(())
    }
}
