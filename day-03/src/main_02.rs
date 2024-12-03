use std::{error::Error, fs};

use regex::Regex;

#[derive(Debug, Clone)]
enum Token {
    Mul,
    Do,
    Dont,
}

#[derive(Debug, Clone)]
struct TRegex {
    token: Token,
    re: Regex,
}

impl TRegex {
    fn new(name: Token, re: Regex) -> Self {
        Self { token: name, re }
    }
}

#[derive(Debug)]
struct TMatch {
    re: TRegex,
    r#match: String,
}

impl TMatch {
    fn new(re: TRegex, r#match: String) -> Self {
        Self { re, r#match }
    }
}

#[derive(Debug)]
struct Matcher {
    regexes: Vec<TRegex>,
}

impl Matcher {
    fn new(regexes: Vec<TRegex>) -> Self {
        Self { regexes }
    }

    fn match_all(&mut self, str: &str) -> MatcherIterator {
        MatcherIterator::new(str, self.regexes.clone())
    }
}

struct MatcherIterator {
    str: String,
    regexes: Vec<TRegex>,
}

impl MatcherIterator {
    fn new(str: &str, regexes: Vec<TRegex>) -> Self {
        Self { str: str.into(), regexes }
    }
}

impl Iterator for MatcherIterator {
    type Item = TMatch;

    fn next(&mut self) -> Option<Self::Item> {
        let mut regexes: Vec<_> =
            self.regexes.clone().into_iter().filter(|re| re.re.is_match(&self.str)).collect();

        if regexes.len() == 0 {
            return None;
        }

        regexes.sort_by_key(|re| re.re.find(&self.str).map(|m| m.start()).unwrap());

        let re = regexes[0].clone();
        let m = re.re.find(&self.str).unwrap();
        let ret = Some(TMatch::new(re, String::from(&self.str[m.start()..m.end()])));
        self.str = String::from(&self.str[m.end()..]);
        ret
    }
}

pub fn main(file: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;

    let mut matcher = Matcher::new(vec![
        TRegex::new(Token::Mul, Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap()),
        TRegex::new(Token::Do, Regex::new(r"do\(\)").unwrap()),
        TRegex::new(Token::Dont, Regex::new(r"don't\(\)").unwrap()),
    ]);

    let mut should_do = true;

    let result = matcher.match_all(&content).fold(0, |sum, m| {
        match m.re.token {
            Token::Do => should_do = true,
            Token::Dont => should_do = false,
            Token::Mul => {
                if should_do {
                    let [s1, s2] = m.re.re.captures(&m.r#match).unwrap().extract().1;
                    let n1: i32 = s1.parse().unwrap();
                    let n2: i32 = s2.parse().unwrap();
                    return sum + n1 * n2;
                }
            }
        };
        sum
    });

    Ok(result)
}
