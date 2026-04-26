//! Shared token metadata used by the lexer NFA and DFA.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Per-rule metadata propagated from the lexer definition into automata.
pub(super) struct TokenMetadata {
    priority: usize,
    id: usize,
}

impl TokenMetadata {
    /// Creates new token metadata.
    pub(super) fn new(priority: usize, id: usize) -> Self {
        Self { priority, id }
    }

    /// Returns the rule priority.
    pub(super) fn priority(&self) -> usize {
        self.priority
    }

    /// Returns the rule identifier.
    pub(super) fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::TokenMetadata;

    #[test]
    fn stores_priority_and_id() {
        let metadata = TokenMetadata::new(3, 17);
        assert_eq!(metadata.priority(), 3);
        assert_eq!(metadata.id(), 17);
    }

    #[test]
    fn is_copy_and_equality_comparable() {
        let a = TokenMetadata::new(1, 2);
        let b = a;
        assert_eq!(a, b);
    }
}