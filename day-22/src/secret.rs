pub struct SecretIterator {
    val: usize,
}

impl SecretIterator {
    pub fn new(init: usize) -> Self {
        Self { val: init }
    }
}

impl Iterator for SecretIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut val = self.val;
        // Calculate the result of multiplying the secret number by 64. Then,
        // mix this result into the secret number. Finally, prune the secret
        // number.
        val = prune(mix(val, val * 64));

        // Calculate the result of dividing the secret number by 32. Round the
        // result down to the nearest integer. Then, mix this result into the
        // secret number. Finally, prune the secret number.
        val = prune(mix(val, val / 32));

        // Calculate the result of multiplying the secret number by 2048. Then,
        // mix this result into the secret number. Finally, prune the secret number.
        val = prune(mix(val, val * 2048));

        self.val = val;
        Some(val)
    }
}

#[inline]
fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

#[inline]
fn prune(a: usize) -> usize {
    a % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_iterator_test() {
        let mut iter = SecretIterator::new(123);
        assert_eq!(iter.next(), Some(15887950));
        assert_eq!(iter.next(), Some(16495136));
        assert_eq!(iter.next(), Some(527345));
        assert_eq!(iter.next(), Some(704524));
        assert_eq!(iter.next(), Some(1553684));
        assert_eq!(iter.next(), Some(12683156));
        assert_eq!(iter.next(), Some(11100544));
        assert_eq!(iter.next(), Some(12249484));
        assert_eq!(iter.next(), Some(7753432));
        assert_eq!(iter.next(), Some(5908254));
    }
}
