use crate::lexer::metadata::TokenMetadata;

pub(crate) struct DfaState {
    pub(crate) transitions: [Option<usize>; 256],
    pub(crate) accept: Option<TokenMetadata>,
}

impl Default for DfaState {
    fn default() -> Self {
        Self {
            transitions: [None; 256],
            accept: None,
        }
    }
}

impl DfaState {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

pub(crate) struct Dfa {
    pub(crate) states: Vec<DfaState>,
    pub(crate) start: usize,
}
