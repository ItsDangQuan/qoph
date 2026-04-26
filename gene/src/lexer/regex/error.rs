use std::{error::Error, fmt};

use crate::span::Location;

#[derive(Debug, Clone, PartialEq)]
/// The error categories emitted by the regex parser.
pub enum RegexParserErrorKind {
    /// The input ended unexpectedly.
    UnexpectedEof,
    /// A grouping was opened but not closed.
    UnclosedGrouping,
    /// A character class was opened but not closed.
    UnclosedCharClass,
    /// A character class without any item.
    EmptyCharClass,
    /// A character range had the wrong ordering.
    InvalidCharRange { start: char, end: char },
    /// A backslash appeared at the end of the input.
    TrailingBackslash,
    /// An unexpected trailing character remained after parsing.
    UnexpectedChar(char),
}

#[derive(Debug, Clone, PartialEq)]
/// A parse error with location information.
pub struct RegexParserError {
    kind: RegexParserErrorKind,
    location: Location,
}

impl RegexParserError {
    /// Creates a new parse error.
    pub fn new(kind: RegexParserErrorKind, location: Location) -> Self {
        Self { kind, location }
    }

    /// Returns the error kind.
    pub fn kind(&self) -> &RegexParserErrorKind {
        &self.kind
    }

    /// Returns the source location at which the error was reported.
    pub fn location(&self) -> Location {
        self.location
    }
}

impl fmt::Display for RegexParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: ",
            self.location.line(),
            self.location.column()
        )?;

        match &self.kind {
            RegexParserErrorKind::UnexpectedEof => {
                write!(f, "Unexpected end of input")
            }
            RegexParserErrorKind::UnclosedGrouping => {
                write!(f, "Unclosed grouping, expected ')'")
            }
            RegexParserErrorKind::UnclosedCharClass => {
                write!(f, "Unclosed character class, expected ']'")
            }
            RegexParserErrorKind::EmptyCharClass => {
                write!(f, "Empty character class")
            }
            RegexParserErrorKind::InvalidCharRange { start, end } => {
                write!(f, "Invalid character range: {}-{}", start, end)
            }
            RegexParserErrorKind::TrailingBackslash => {
                write!(f, "Trailing backslash '\\' with no escaped character")
            }
            RegexParserErrorKind::UnexpectedChar(c) => {
                write!(f, "Unexpected character '{}'", c)
            }
        }
    }
}

impl Error for RegexParserError {}

/// The result type used throughout the regex parser.
pub type RegexParserResult<T> = Result<T, RegexParserError>;
