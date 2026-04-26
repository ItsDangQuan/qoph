//! Lexer-definition parsing internals.

mod ast;
mod error;
mod parser;

pub(crate) use ast::{LexerDefinition, PayloadType};
pub(crate) use parser::DefinitionParser;
