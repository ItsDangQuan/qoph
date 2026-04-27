use crate::lexer::metadata::TokenMetadata;

pub(crate) enum Transition {
    Epsilon(usize, Option<usize>),
    MatchRange(u8, u8, usize),
    Accept(TokenMetadata),
    DeadEnd,
}

pub(crate) struct NfaState {
    pub(crate) transition: Transition,
}

pub(crate) struct Nfa {
    pub(crate) states: Vec<NfaState>,
    pub(crate) start: usize,
}
