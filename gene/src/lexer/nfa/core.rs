use crate::lexer::metadata::TokenMetadata;

pub(crate) enum Transition {
    Epsilon(usize, Option<usize>),
    MatchRange(u8, u8, usize),
    Accept(TokenMetadata),
    DeadEnd,
}

pub(crate) struct NfaState {
    pub(super) transition: Transition,
}

impl NfaState {
    pub(crate) fn transition(&self) -> &Transition {
        &self.transition
    }
}

pub(crate) struct Nfa {
    pub(super) states: Vec<NfaState>,
    pub(super) start: usize,
}

impl Nfa {
    pub(crate) fn states(&self) -> &[NfaState] {
        &self.states
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }
}
