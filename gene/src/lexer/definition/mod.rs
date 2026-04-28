//! Lexer-definition parsing internals.

mod core;
mod error;
mod lexer;
mod parser;

pub(crate) use core::{LexerDefinition, PayloadType};
pub(crate) use parser::DefinitionParser;
