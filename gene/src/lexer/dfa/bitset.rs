//! Bitset utilities used by DFA subset construction.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct BitSet(Vec<u64>);

impl BitSet {
    pub(super) fn new(words: usize) -> Self {
        Self(vec![0; words])
    }

    pub(super) fn set(&mut self, index: usize) {
        self.0[index / 64] |= 1 << (index % 64);
    }

    pub(super) fn test(&self, index: usize) -> bool {
        (self.0[index / 64] & (1 << (index % 64))) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;

    #[test]
    fn set_and_test_work_across_word_boundaries() {
        let mut bits = BitSet::new(3);
        bits.set(0);
        bits.set(63);
        bits.set(64);
        bits.set(127);

        assert!(bits.test(0));
        assert!(bits.test(63));
        assert!(bits.test(64));
        assert!(bits.test(127));
        assert!(!bits.test(1));
        assert!(!bits.test(126));
    }

    #[test]
    fn ordering_and_equality_are_stable() {
        let mut a = BitSet::new(2);
        let mut b = BitSet::new(2);
        a.set(5);
        b.set(5);
        assert_eq!(a, b);

        b.set(65);
        assert!(a < b);
    }
}
