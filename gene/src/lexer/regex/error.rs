use std::{error::Error, fmt, ops::Range};

#[derive(Debug, Clone, PartialEq)]
/// The error categories emitted by the regex parser.
pub(crate) enum RegexParserErrorKind {
    UnexpectedTrailingChars,
    UnclosedGrouping,
    UnclosedCharClass,
    UnclosedFragmentRef,
    InvalidCharRange { start: char, end: char },
    UnescapedMetaChar(char),
    UnescapedClassMetaChar(char),
    TrailingBackslash,
    UnexpectedEof,
    EmptyFragmentRef,
    InvalidIdentStart(char),
}

#[derive(Debug, Clone, PartialEq)]
/// A parse error with location information.
pub(crate) struct RegexParserError {
    pub(crate) kind: RegexParserErrorKind,
    pub(crate) span: Range<usize>,
}

impl fmt::Display for RegexParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Regex parse error at {}-{}: ",
            self.span.start, self.span.end
        )?;

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
            RegexParserErrorKind::UnclosedFragmentRef => {
                write!(f, "Unclosed fragment reference, expected '}}'")
            }
            RegexParserErrorKind::EmptyFragmentRef => {
                write!(f, "Empty fragment reference, expected identifier")
            }
            RegexParserErrorKind::InvalidIdentStart(c) => {
                write!(
                    f,
                    "Invalid identifier start '{}', expected ASCII alphabetic or underscore '_'",
                    c
                )
            }
        }
    }
}

impl Error for RegexParserError {}
