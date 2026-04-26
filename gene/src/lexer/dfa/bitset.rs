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
