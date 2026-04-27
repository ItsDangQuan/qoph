use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
/// The error categories emitted by the regex parser.
pub enum RegexParserErrorKind {
    /// The input ended unexpectedly.
    UnexpectedEof,
    /// A grouping was opened but not closed.
    UnclosedGrouping,
    /// A character class was opened but not closed.
    UnclosedCharClass,
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
    location: usize,
}

impl RegexParserError {
    /// Creates a new parse error.
    pub fn new(kind: RegexParserErrorKind, location: usize) -> Self {
        Self { kind, location }
    }

    #[allow(unused)]
    /// Returns the error kind.
    pub fn kind(&self) -> &RegexParserErrorKind {
        &self.kind
    }

    #[allow(unused)]
    /// Returns the byte offset at which the error was reported.
    pub fn location(&self) -> usize {
        self.location
    }
}

impl fmt::Display for RegexParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at byte {}: ", self.location)?;

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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{RegexParserError, RegexParserErrorKind};

    fn assert_is_error_trait<E: Error>() {}

    #[test]
    fn accessors_return_constructor_data() {
        let err = RegexParserError::new(RegexParserErrorKind::UnexpectedChar('x'), 7);
        assert!(matches!(
            err.kind(),
            RegexParserErrorKind::UnexpectedChar('x')
        ));
        assert_eq!(err.location(), 7);
    }

    #[test]
    fn display_formats_all_error_variants() {
        let cases = vec![
            (
                RegexParserErrorKind::UnexpectedEof,
                "Unexpected end of input",
            ),
            (
                RegexParserErrorKind::UnclosedGrouping,
                "Unclosed grouping, expected ')'",
            ),
            (
                RegexParserErrorKind::UnclosedCharClass,
                "Unclosed character class, expected ']'",
            ),
            (
                RegexParserErrorKind::InvalidCharRange {
                    start: 'z',
                    end: 'a',
                },
                "Invalid character range: z-a",
            ),
            (
                RegexParserErrorKind::TrailingBackslash,
                "Trailing backslash '\\' with no escaped character",
            ),
            (
                RegexParserErrorKind::UnexpectedChar('!'),
                "Unexpected character '!'",
            ),
        ];

        for (kind, tail) in cases {
            let err = RegexParserError::new(kind, 9);
            let msg = err.to_string();
            assert!(msg.starts_with("Parse error at byte 9: "));
            assert!(msg.contains(tail));
        }
    }

    #[test]
    fn type_implements_std_error() {
        assert_is_error_trait::<RegexParserError>();
    }
}
