//! Shared token metadata used by the lexer NFA and DFA.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Per-rule metadata propagated from the lexer definition into automata.
pub struct TokenMetadata {
    priority: usize,
    id: usize,
}

impl TokenMetadata {
    /// Creates new token metadata.
    pub fn new(priority: usize, id: usize) -> Self {
        Self { priority, id }
    }

    /// Returns the rule priority.
    pub fn priority(&self) -> usize {
        self.priority
    }

    /// Returns the rule identifier.
    pub fn id(&self) -> usize {
        self.id
    }
}