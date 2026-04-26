//! DFA construction for lexer rules.

mod bitset;
mod builder;
mod core;
mod optimizer;

pub(crate) use builder::DfaBuilder;
pub(crate) use core::Dfa;
pub(crate) use optimizer::DfaOptimizer;
