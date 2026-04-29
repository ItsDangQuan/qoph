use std::{error::Error, fmt, ops::Range};

/// The exposed error kinds for diagnostic, enumerating all possible errors during code generation.
// Internally, individual pass don't inspect the errors of previous pass, and they all are either
// fail-fast or error-accumulating. As thus, this error design can be justified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerCompilerError {
    // Lexing errors
    UnrecognizedToken,

    // Parsing errors
    UnexpectedToken { expected: String, found: String },
    UnexpectedEof { expected: String },

    // Regex parsing errors
    RegexUnexpectedTrailingChars,
    RegexUnclosedGrouping,
    RegexUnclosedCharClass,
    RegexUnclosedFragmentRef,
    RegexInvalidCharRange { start: char, end: char },
    RegexUnescapedMetaChar(char),
    RegexUnescapedClassMetaChar(char),
    RegexTrailingBackslash,
    RegexUnexpectedEof,
    RegexEmptyFragmentRef,
    RegexInvalidIdentStart(char),

    // Semantic errors
    CollidingName { name: String, span: Range<usize> },
    UndefinedFragment(String),
}

impl fmt::Display for LexerCompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerCompilerError::UnrecognizedToken => {
                // can be improved
                write!(f, "Found unrecognized character or unclosed sequence")
            }
            LexerCompilerError::UnexpectedToken { expected, found } => {
                write!(f, "Found {}, expected {}", found, expected)
            }
            LexerCompilerError::UnexpectedEof { expected } => {
                write!(f, "Found end of file, expected {}", expected)
            }
            LexerCompilerError::RegexUnexpectedTrailingChars => {
                write!(f, "Found unexpected trailing character after expression")
            }
            LexerCompilerError::RegexUnclosedGrouping => {
                write!(f, "Found unclosed grouping, expected ')'")
            }
            LexerCompilerError::RegexUnclosedCharClass => {
                write!(f, "Found unclosed character class, expected ']'")
            }
            LexerCompilerError::RegexUnclosedFragmentRef => {
                write!(f, "Found unclosed fragment reference, expected '}}'")
            }
            LexerCompilerError::RegexInvalidCharRange { start, end } => {
                write!(f, "Found invalid character range '{start}-{end}'")
            }
            LexerCompilerError::RegexUnescapedMetaChar(c) => {
                write!(f, "Found unescaped metacharacter '{}'", c)
            }
            LexerCompilerError::RegexUnescapedClassMetaChar(c) => {
                write!(f, "Found unescaped class metacharacter '{}'", c)
            }
            LexerCompilerError::RegexTrailingBackslash => {
                write!(f, "Found trailing backslash '\\' with no escaped character")
            }
            LexerCompilerError::RegexUnexpectedEof => {
                write!(f, "Found unexpected end of regular expression")
            }
            LexerCompilerError::RegexEmptyFragmentRef => {
                write!(f, "Found empty fragment reference, expected identifier")
            }
            LexerCompilerError::RegexInvalidIdentStart(c) => {
                write!(f, "Found invalid identifier start '{}'", c)
            }
            LexerCompilerError::CollidingName { name, .. } => {
                write!(f, "Found colliding name '{}'", name)
            }
            LexerCompilerError::UndefinedFragment(name) => {
                write!(f, "Found undefined fragment '{}'", name)
            }
        }
    }
}

impl Error for LexerCompilerError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerCompilerDiag {
    error: LexerCompilerError,
    span: Range<usize>,
}

impl LexerCompilerDiag {
    pub fn error(&self) -> &LexerCompilerError {
        &self.error
    }

    pub fn span(&self) -> &Range<usize> {
        &self.span
    }
}

impl fmt::Display for LexerCompilerDiag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at byte range {}..{}: {}",
            self.span.start, self.span.end, self.error
        )
    }
}

impl Error for LexerCompilerDiag {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
