//! Shared token metadata used by the lexer NFA and DFA.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Per-rule metadata propagated from the lexer definition into automata.
pub(crate) struct TokenMetadata {
    pub(crate) priority: usize,
    pub(crate) id: usize,
}
