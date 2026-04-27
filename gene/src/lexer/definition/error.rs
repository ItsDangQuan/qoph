use std::{error::Error, fmt};

use crate::lexer::regex::RegexParserError;

#[derive(Debug)]
pub(super) enum DefinitionParserErrorKind {
    NoRules,
    MissingDelimiter,
    EmptyRegex,
    EmptyCategory,
    UnclosedPayload,
    MismatchedParentheses,
    UnknownPayloadType(String),
    RegexParseError(RegexParserError),
}

#[derive(Debug)]
pub(crate) struct DefinitionParserError {
    kind: DefinitionParserErrorKind,
    location: usize,
}

impl DefinitionParserError {
    pub(super) fn new(kind: DefinitionParserErrorKind, location: usize) -> Self {
        Self { kind, location }
    }

    #[allow(unused)]
    pub(super) fn kind(&self) -> &DefinitionParserErrorKind {
        &self.kind
    }

    #[allow(unused)]
    pub(super) fn location(&self) -> usize {
        self.location
    }
}

impl fmt::Display for DefinitionParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(&self.kind, DefinitionParserErrorKind::NoRules) {
            return match &self.kind {
                DefinitionParserErrorKind::NoRules => {
                    write!(f, "Lexer definition contains no valid rules.")
                }
                _ => write!(f, "Global error: {:?}", self.kind),
            };
        }

        write!(f, "Error at byte {}: ", self.location)?;

        match &self.kind {
            DefinitionParserErrorKind::MissingDelimiter => {
                write!(f, "Missing '::=' delimiter.")
            }
            DefinitionParserErrorKind::EmptyRegex => {
                write!(f, "Empty regular expression.")
            }
            DefinitionParserErrorKind::EmptyCategory => {
                write!(f, "Missing token variant name.")
            }
            DefinitionParserErrorKind::UnclosedPayload => {
                write!(f, "Unclosed payload parenthesis.")
            }
            DefinitionParserErrorKind::MismatchedParentheses => {
                write!(f, "Mismatched payload parentheses.")
            }
            DefinitionParserErrorKind::UnknownPayloadType(t) => {
                write!(f, "Unknown payload type '{}'.", t)
            }
            DefinitionParserErrorKind::RegexParseError(e) => {
                write!(f, "Regex syntax error: {}", e)
            }
            DefinitionParserErrorKind::NoRules => unreachable!(),
        }
    }
}

impl Error for DefinitionParserError {}

pub(super) type DefinitionParserResult<T> = Result<T, DefinitionParserError>;

#[cfg(test)]
mod tests {
    use super::{DefinitionParserError, DefinitionParserErrorKind};

    #[test]
    fn accessors_return_constructor_data() {
        let err = DefinitionParserError::new(DefinitionParserErrorKind::MissingDelimiter, 8);

        assert!(matches!(
            err.kind(),
            DefinitionParserErrorKind::MissingDelimiter
        ));
        assert_eq!(err.location(), 8);
    }
}
