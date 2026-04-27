//! Regular-expression parsing support for lexer definitions.
//!
//! Grammar overview:
//!
//! ```text
//! Expression      ::= Alternation
//! Alternation     ::= Concatenation ( '|' Concatenation )*
//! Concatenation   ::= Closure ( Closure )*
//! Closure         ::= Base ( '*' | '+' | '?' )?
//! Base            ::= Literal | '.' | EscapeSequence | CharacterClass | '(' Expression ')'
//! CharacterClass  ::= '[' '^'? ClassItem+ ']'
//! ClassItem       ::= ClassChar '-' ClassChar | ClassChar | EscapeSequence
//! ClassChar       ::= any character except ']' and '\'
//! EscapeSequence   ::= '\' ( 'n' | 't' | 'r' | any other character )
//! Literal         ::= any character except control characters '|', '*', '+', '?', '(', ')', '[', '\', '.'
//! ```

mod ast;
mod error;
mod parser;

pub(crate) use ast::{RegexAst, RegexAstKind};
pub(crate) use error::RegexParserError;
pub(crate) use parser::RegexParser;
