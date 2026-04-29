//! Lexer-definition parsing internals.

mod core;
mod lexer;
mod parser;

pub(crate) use core::{FragmentRule, Rule, RuleId, TokenRule};
pub(crate) use parser::RuleParser;
