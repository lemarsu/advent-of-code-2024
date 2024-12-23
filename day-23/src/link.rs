use std::str::FromStr;

#[derive(Debug, Clone, Eq)]
pub struct Link {
    pub a: String,
    pub b: String,
}

impl Link {
    pub fn new(a: String, b: String) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn is_linked_to(&self, str: &str) -> bool {
        self.a == str || self.b == str
    }

    pub fn share_connection(&self, other: &Self) -> bool {
        self.is_linked_to(&other.a) || self.is_linked_to(&other.b)
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.is_linked_to(&other.a) & self.is_linked_to(&other.b)
    }
}

impl FromStr for Link {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("-").collect();
        let a = parts[0].into();
        let b = parts[1].into();
        Ok(Self::new(a, b))
    }
}

impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.a == other.a {
            self.b.partial_cmp(&other.b)
        } else {
            self.a.partial_cmp(&other.a)
        }
    }
}

impl Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.a == other.a {
            self.b.cmp(&other.b)
        } else {
            self.a.cmp(&other.a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn couple_eq_test() {
        assert_eq!(Link::new("a".into(), "b".into()), Link::new("a".into(), "b".into()));
        assert_eq!(Link::new("a".into(), "b".into()), Link::new("b".into(), "a".into()));
        assert_ne!(Link::new("a".into(), "b".into()), Link::new("b".into(), "c".into()));
        assert_ne!(Link::new("a".into(), "b".into()), Link::new("c".into(), "b".into()));
    }
}
