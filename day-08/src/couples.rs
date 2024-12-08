pub struct CouplesIterator<T: Clone> {
    src: Vec<T>,
    posa: usize,
    posb: usize,
}

impl<T: Clone> CouplesIterator<T> {
    pub fn new(src: &Vec<T>) -> Self {
        Self { src: src.clone(), posa: 0, posb: 0 }
    }
}

impl<T: Clone> Iterator for CouplesIterator<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.src.len();

        if self.posa >= len {
            return None;
        }
        if self.posb >= len {
            self.posa += 1;
            self.posb = self.posa;
            return self.next();
        }
        if self.posa == self.posb {
            self.posb += 1;
            return self.next();
        }
        let a = self.src[self.posa].clone();
        let b = self.src[self.posb].clone();
        self.posb += 1;
        Some((a, b))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn couple_iterator_test() {
        let src = vec![1, 2, 3];
        let iterator = CouplesIterator::new(&src);
        let result: HashSet<_> = iterator.collect();
        assert_eq!(result.len(), 3);
        assert!(result.contains(&(1, 2)));
        assert!(result.contains(&(1, 3)));
        assert!(result.contains(&(2, 3)));
    }
}
