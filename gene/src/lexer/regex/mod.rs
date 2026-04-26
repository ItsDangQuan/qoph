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

pub(super) use ast::{CharSet, RegexAst, RegexAstKind};
pub(super) use error::{RegexParserError, RegexParserErrorKind, RegexParserResult};
pub(super) use parser::RegexParser;
