//! Regular-expression parsing support for lexer definitions.
//!
//! Grammar overview:
//!
//! ```peg
//! Expression     <- Alternation !.
//! Alternation    <- Concatenation ( '|' Concatenation )*
//! Concatenation  <- Closure ( !( '|' / ')' ) Closure )*
//! Closure        <- Base ( '*' / '+' / '?' )?
//! Base           <- '(' Alternation ')'
//!                 / '[' CharacterClass ']'
//!                 / '{' [a-zA-Z_] [a-zA-Z0-9_]* '}'
//!                 / '\\' EscapeCode
//!                 / '.'
//!                 / !( '|' / '*' / '+' / '?' / ')' / ']' / '}' ) .
//! CharacterClass <- '^'? ( !']' ClassItem )*
//! ClassItem      <- ClassBound ( '-' ClassBound )?
//! ClassBound     <- '\\' EscapeCode
//!                 / !( ']' / '-' ) .
//! EscapeCode     <- 'n' / 't' / 'r' / .
//! ```

mod char_set;
mod core;
mod error;
mod parser;

pub(crate) use core::RegexAst;
pub(crate) use error::RegexParserError;
pub(crate) use parser::RegexParser;
