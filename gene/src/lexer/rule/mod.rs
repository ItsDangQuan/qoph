//! Lexer-definition parsing internals.

mod core;
mod lexer;
mod parser;

pub(crate) use core::{Rule, RuleId};
pub(crate) use parser::RuleParser;
