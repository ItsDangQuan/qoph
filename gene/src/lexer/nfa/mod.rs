//! NFA construction for lexer rules.

mod builder;
mod byte;
mod core;

pub(crate) use builder::NfaBuilder;
pub(crate) use core::{Nfa, Transition};
