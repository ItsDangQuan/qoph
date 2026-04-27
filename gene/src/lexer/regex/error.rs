use std::{error::Error, fmt, ops::Range};

#[derive(Debug, Clone, PartialEq)]
/// The error categories emitted by the regex parser.
pub(crate) enum RegexParserErrorKind {
    UnexpectedTrailingChars,
    UnclosedGrouping,
    UnclosedCharClass,
    InvalidCharRange { start: char, end: char },
    UnescapedMetaChar(char),
    UnescapedClassMetaChar(char),
    TrailingBackslash,
    UnexpectedEof,
}

#[derive(Debug, Clone, PartialEq)]
/// A parse error with location information.
pub(crate) struct RegexParserError {
    pub(crate) kind: RegexParserErrorKind,
    pub(crate) span: Range<usize>,
}

impl fmt::Display for RegexParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at {}-{}: ", self.span.start, self.span.end)?;

        match &self.kind {
            RegexParserErrorKind::UnexpectedTrailingChars => {
                write!(f, "Unexpected trailing characters after expression")
            }
            RegexParserErrorKind::UnclosedGrouping => {
                write!(f, "Unclosed grouping, expected ')'")
            }
            RegexParserErrorKind::UnclosedCharClass => {
                write!(f, "Unclosed character class, expected ']'")
            }
            RegexParserErrorKind::InvalidCharRange { start, end } => {
                write!(f, "Invalid character range: {}-{}", start, end)
            }
            RegexParserErrorKind::UnescapedMetaChar(c) => {
                write!(f, "Unescaped metacharacter '{}'", c)
            }
            RegexParserErrorKind::UnescapedClassMetaChar(c) => {
                write!(f, "Unescaped class metacharacter '{}'", c)
            }
            RegexParserErrorKind::TrailingBackslash => {
                write!(f, "Trailing backslash '\\' with no escaped character")
            }
            RegexParserErrorKind::UnexpectedEof => {
                write!(f, "Unexpected end of input")
            }
        }
    }
}

impl Error for RegexParserError {}
