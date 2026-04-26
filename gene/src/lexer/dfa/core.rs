use crate::lexer::metadata::TokenMetadata;

pub(crate) struct DfaState {
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

    pub(crate) fn transitions(&self) -> &[Option<usize>; 256] {
        &self.transitions
    }

    pub(crate) fn accept(&self) -> Option<TokenMetadata> {
        self.accept
    }
}

pub(crate) struct Dfa {
    pub(super) states: Vec<DfaState>,
    pub(super) start: usize,
}

impl Dfa {
    pub(crate) fn states(&self) -> &[DfaState] {
        &self.states
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }
}
