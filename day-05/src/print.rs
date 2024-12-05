use std::str::FromStr;

use crate::{page_list::PageList, rule::Rule};

#[derive(Debug, Clone, Default)]
pub struct Print {
    pub rules: Vec<Rule>,
    pub page_lists: Vec<PageList>,
}

impl Print {
    pub fn get_valid_lists(&self) -> Vec<PageList> {
        self.page_lists
            .iter()
            .filter(|list| list.respect_rules(&self.rules))
            .map(|list| list.clone())
            .collect()
    }

    pub fn get_invalid_lists(&self) -> Vec<PageList> {
        self.page_lists
            .iter()
            .filter(|list| !list.respect_rules(&self.rules))
            .map(|list| list.clone())
            .collect()
    }
}

impl FromStr for Print {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules_done = false;
        let mut ret = Self::default();
        for line in s.split("\n") {
            if line == "" {
                if !rules_done {
                    rules_done = true;
                }
                continue;
            }
            if !rules_done {
                ret.rules.push(line.parse().unwrap());
            } else {
                ret.page_lists.push(line.parse().unwrap());
            }
        }
        Ok(ret)
    }
}
