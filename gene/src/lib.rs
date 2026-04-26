//! Gene generates lexer and parser code from their definition.

mod lexer;
mod span;

pub use lexer::generate_lexer_source;
