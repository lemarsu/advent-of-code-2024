use std::{cmp::Ordering, str::FromStr};

use crate::rule::Rule;

#[derive(Debug, Clone, Default)]
pub struct PageList(Vec<i32>);

impl PageList {
    pub fn middle(&self) -> i32 {
        self.0[self.0.len() / 2]
    }

    pub fn respect_rules(&self, rules: &Vec<Rule>) -> bool {
        for rule in rules.iter() {
            let first_pos = self.0.iter().position(|i| i == &rule.0);
            let last_pos = self.0.iter().position(|i| i == &rule.1);
            match (first_pos, last_pos) {
                (Some(first), Some(last)) => {
                    if first > last {
                        return false;
                    }
                },
                _ => (),
            }
        }
        true
    }

    pub fn sort_with_rules(&mut self, rules: &Vec<Rule>) {
        self.0.sort_by(|a, b| {
            if let Some(rule) =
                rules.iter().find(|r| &r.0 == a && &r.1 == b || &r.0 == b && &r.1 == a)
            {
                return if &rule.0 == a { Ordering::Less } else { Ordering::Greater };
            } else {
                Ordering::Equal
            }
        })
    }
}

impl FromStr for PageList {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s.split(",").map(|num| num.parse().unwrap()).collect();
        Ok(Self(nums))
    }
}
