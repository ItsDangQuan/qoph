use crate::lexer::metadata::TokenMetadata;

pub(super) struct DfaState {
    pub(super) transitions: [Option<usize>; 256],
    pub(super) accept: Option<TokenMetadata>,
}

impl DfaState {
    pub(super) fn new() -> Self {
        Self {
            transitions: [None; 256],
            accept: None,
        }
    }
}

pub(super) struct Dfa {
    pub(super) states: Vec<DfaState>,
    pub(super) start: usize,
}