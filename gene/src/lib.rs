//! Gene generates lexer and parser code from their definition.

mod lexer;

pub use lexer::{LexerCompilerOptions, generate_lexer_source};
