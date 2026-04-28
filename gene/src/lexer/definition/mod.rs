//! Lexer-definition parsing internals.

mod core;
mod error;
#[rustfmt::skip]
mod lexer;
mod parser;

pub(crate) use core::{LexerDef, RuleId};
pub(crate) use parser::DefinitionParser;
