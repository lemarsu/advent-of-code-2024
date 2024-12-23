#[derive(Debug, Clone)]
pub enum Kind {
    Each,
    Distinct,
}

impl Kind {
    fn indexes(&self, size: usize) -> Vec<usize> {
        match self {
            Self::Each => (0..size).map(|_| 0).collect(),
            Self::Distinct => (0..size).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Combinator<T: Clone> {
    kind: Kind,
    items: Vec<T>,
    size: usize,
    indexes: Vec<usize>,
}

impl<T: Clone> Combinator<T> {
    fn new<I: IntoIterator<Item = T>>(kind: Kind, items: I, size: usize) -> Self {
        let items: Vec<_> = items.into_iter().collect();
        let indexes = if items.len() < size { Vec::new() } else { kind.indexes(size) };
        Self { kind, items, size, indexes }
    }

    pub fn each<I: IntoIterator<Item = T>>(items: I, size: usize) -> Self {
        Self::new(Kind::Each, items, size)
    }

    pub fn distinct<I: IntoIterator<Item = T>>(items: I, size: usize) -> Self {
        Self::new(Kind::Distinct, items, size)
    }

    fn update_indexes(&mut self) {
        for i in (0..self.size).rev() {
            let mut remainder = false;
            let idx = &mut self.indexes[i];
            *idx += 1;
            if *idx >= self.items.len() && i != 0 {
                *idx = 0;
                remainder = true;
            }
            if !remainder {
                return;
            }
        }
    }

    fn update_indexes_distinctly(&mut self) {
        'upper: loop {
            self.update_indexes();
            for i in 0..self.size {
                if self.indexes[(i + 1)..].contains(&self.indexes[i]) {
                    continue 'upper;
                }
            }
            break;
        }
    }
}

impl<T: Clone> Iterator for Combinator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes.is_empty() || self.indexes.iter().any(|i| *i >= self.items.len()) {
            return None;
        }
        let ret = Some(self.indexes.iter().map(|i| self.items[*i].clone()).collect());
        match self.kind {
            Kind::Each => self.update_indexes(),
            Kind::Distinct => self.update_indexes_distinctly(),
        };
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_iterator_test() {
        let mut iter = Combinator::each([1], 2);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::each([1, 2, 3], 1);

        assert_eq!(iter.next(), Some(vec![1]));
        assert_eq!(iter.next(), Some(vec![2]));
        assert_eq!(iter.next(), Some(vec![3]));
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::each([1, 2, 3], 2);
        assert_eq!(iter.next(), Some(vec![1, 1]));
        assert_eq!(iter.next(), Some(vec![1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 1]));
        assert_eq!(iter.next(), Some(vec![2, 2]));
        assert_eq!(iter.next(), Some(vec![2, 3]));
        assert_eq!(iter.next(), Some(vec![3, 1]));
        assert_eq!(iter.next(), Some(vec![3, 2]));
        assert_eq!(iter.next(), Some(vec![3, 3]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::each([1, 2, 3], 3);
        assert_eq!(iter.next(), Some(vec![1, 1, 1]));
        assert_eq!(iter.next(), Some(vec![1, 1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 1, 3]));
        assert_eq!(iter.next(), Some(vec![1, 2, 1]));
        assert_eq!(iter.next(), Some(vec![1, 2, 2]));
        assert_eq!(iter.next(), Some(vec![1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![1, 3, 1]));
        assert_eq!(iter.next(), Some(vec![1, 3, 2]));
        assert_eq!(iter.next(), Some(vec![1, 3, 3]));
        assert_eq!(iter.next(), Some(vec![2, 1, 1]));
        assert_eq!(iter.next(), Some(vec![2, 1, 2]));
        assert_eq!(iter.next(), Some(vec![2, 1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 2, 1]));
        assert_eq!(iter.next(), Some(vec![2, 2, 2]));
        assert_eq!(iter.next(), Some(vec![2, 2, 3]));
        assert_eq!(iter.next(), Some(vec![2, 3, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3, 2]));
        assert_eq!(iter.next(), Some(vec![2, 3, 3]));
        assert_eq!(iter.next(), Some(vec![3, 1, 1]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2]));
        assert_eq!(iter.next(), Some(vec![3, 1, 3]));
        assert_eq!(iter.next(), Some(vec![3, 2, 1]));
        assert_eq!(iter.next(), Some(vec![3, 2, 2]));
        assert_eq!(iter.next(), Some(vec![3, 2, 3]));
        assert_eq!(iter.next(), Some(vec![3, 3, 1]));
        assert_eq!(iter.next(), Some(vec![3, 3, 2]));
        assert_eq!(iter.next(), Some(vec![3, 3, 3]));
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::each([1, 2, 3, 4], 3);
        assert_eq!(iter.next(), Some(vec![1, 1, 1]));
        assert_eq!(iter.next(), Some(vec![1, 1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 1, 3]));
        assert_eq!(iter.next(), Some(vec![1, 1, 4]));
        assert_eq!(iter.next(), Some(vec![1, 2, 1]));
        assert_eq!(iter.next(), Some(vec![1, 2, 2]));
        assert_eq!(iter.next(), Some(vec![1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![1, 2, 4]));
        assert_eq!(iter.next(), Some(vec![1, 3, 1]));
        assert_eq!(iter.next(), Some(vec![1, 3, 2]));
        assert_eq!(iter.next(), Some(vec![1, 3, 3]));
        assert_eq!(iter.next(), Some(vec![1, 3, 4]));
        assert_eq!(iter.next(), Some(vec![1, 4, 1]));
        assert_eq!(iter.next(), Some(vec![1, 4, 2]));
        assert_eq!(iter.next(), Some(vec![1, 4, 3]));
        assert_eq!(iter.next(), Some(vec![1, 4, 4]));
        assert_eq!(iter.next(), Some(vec![2, 1, 1]));
        assert_eq!(iter.next(), Some(vec![2, 1, 2]));
        assert_eq!(iter.next(), Some(vec![2, 1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 1, 4]));
        assert_eq!(iter.next(), Some(vec![2, 2, 1]));
        assert_eq!(iter.next(), Some(vec![2, 2, 2]));
        assert_eq!(iter.next(), Some(vec![2, 2, 3]));
        assert_eq!(iter.next(), Some(vec![2, 2, 4]));
        assert_eq!(iter.next(), Some(vec![2, 3, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3, 2]));
        assert_eq!(iter.next(), Some(vec![2, 3, 3]));
        assert_eq!(iter.next(), Some(vec![2, 3, 4]));
        assert_eq!(iter.next(), Some(vec![2, 4, 1]));
        assert_eq!(iter.next(), Some(vec![2, 4, 2]));
        assert_eq!(iter.next(), Some(vec![2, 4, 3]));
        assert_eq!(iter.next(), Some(vec![2, 4, 4]));
        assert_eq!(iter.next(), Some(vec![3, 1, 1]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2]));
        assert_eq!(iter.next(), Some(vec![3, 1, 3]));
        assert_eq!(iter.next(), Some(vec![3, 1, 4]));
        assert_eq!(iter.next(), Some(vec![3, 2, 1]));
        assert_eq!(iter.next(), Some(vec![3, 2, 2]));
        assert_eq!(iter.next(), Some(vec![3, 2, 3]));
        assert_eq!(iter.next(), Some(vec![3, 2, 4]));
        assert_eq!(iter.next(), Some(vec![3, 3, 1]));
        assert_eq!(iter.next(), Some(vec![3, 3, 2]));
        assert_eq!(iter.next(), Some(vec![3, 3, 3]));
        assert_eq!(iter.next(), Some(vec![3, 3, 4]));
        assert_eq!(iter.next(), Some(vec![3, 4, 1]));
        assert_eq!(iter.next(), Some(vec![3, 4, 2]));
        assert_eq!(iter.next(), Some(vec![3, 4, 3]));
        assert_eq!(iter.next(), Some(vec![3, 4, 4]));
        assert_eq!(iter.next(), Some(vec![4, 1, 1]));
        assert_eq!(iter.next(), Some(vec![4, 1, 2]));
        assert_eq!(iter.next(), Some(vec![4, 1, 3]));
        assert_eq!(iter.next(), Some(vec![4, 1, 4]));
        assert_eq!(iter.next(), Some(vec![4, 2, 1]));
        assert_eq!(iter.next(), Some(vec![4, 2, 2]));
        assert_eq!(iter.next(), Some(vec![4, 2, 3]));
        assert_eq!(iter.next(), Some(vec![4, 2, 4]));
        assert_eq!(iter.next(), Some(vec![4, 3, 1]));
        assert_eq!(iter.next(), Some(vec![4, 3, 2]));
        assert_eq!(iter.next(), Some(vec![4, 3, 3]));
        assert_eq!(iter.next(), Some(vec![4, 3, 4]));
        assert_eq!(iter.next(), Some(vec![4, 4, 1]));
        assert_eq!(iter.next(), Some(vec![4, 4, 2]));
        assert_eq!(iter.next(), Some(vec![4, 4, 3]));
        assert_eq!(iter.next(), Some(vec![4, 4, 4]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn each_distinct_iterator_test() {
        let mut iter = Combinator::distinct([1], 2);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::distinct([1, 2, 3], 1);

        assert_eq!(iter.next(), Some(vec![1]));
        assert_eq!(iter.next(), Some(vec![2]));
        assert_eq!(iter.next(), Some(vec![3]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::distinct([1, 2, 3], 2);
        assert_eq!(iter.next(), Some(vec![1, 2]));
        assert_eq!(iter.next(), Some(vec![1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3]));
        assert_eq!(iter.next(), Some(vec![3, 1]));
        assert_eq!(iter.next(), Some(vec![3, 2]));
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::distinct([1, 2, 3], 3);
        assert_eq!(iter.next(), Some(vec![1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![1, 3, 2]));
        assert_eq!(iter.next(), Some(vec![2, 1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 3, 1]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2]));
        assert_eq!(iter.next(), Some(vec![3, 2, 1]));
        assert_eq!(iter.next(), None);

        let mut iter = Combinator::distinct([1, 2, 3, 4], 3);
        assert_eq!(iter.next(), Some(vec![1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![1, 2, 4]));
        assert_eq!(iter.next(), Some(vec![1, 3, 2]));
        assert_eq!(iter.next(), Some(vec![1, 3, 4]));
        assert_eq!(iter.next(), Some(vec![1, 4, 2]));
        assert_eq!(iter.next(), Some(vec![1, 4, 3]));
        assert_eq!(iter.next(), Some(vec![2, 1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 1, 4]));
        assert_eq!(iter.next(), Some(vec![2, 3, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3, 4]));
        assert_eq!(iter.next(), Some(vec![2, 4, 1]));
        assert_eq!(iter.next(), Some(vec![2, 4, 3]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2]));
        assert_eq!(iter.next(), Some(vec![3, 1, 4]));
        assert_eq!(iter.next(), Some(vec![3, 2, 1]));
        assert_eq!(iter.next(), Some(vec![3, 2, 4]));
        assert_eq!(iter.next(), Some(vec![3, 4, 1]));
        assert_eq!(iter.next(), Some(vec![3, 4, 2]));
        assert_eq!(iter.next(), Some(vec![4, 1, 2]));
        assert_eq!(iter.next(), Some(vec![4, 1, 3]));
        assert_eq!(iter.next(), Some(vec![4, 2, 1]));
        assert_eq!(iter.next(), Some(vec![4, 2, 3]));
        assert_eq!(iter.next(), Some(vec![4, 3, 1]));
        assert_eq!(iter.next(), Some(vec![4, 3, 2]));
        assert_eq!(iter.next(), None);
    }
}
