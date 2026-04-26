use std::{error::Error, fmt};

use crate::{lexer::regex::RegexParserError, span::Location};

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
    location: Location,
}

impl DefinitionParserError {
    pub(super) fn new(kind: DefinitionParserErrorKind, location: Location) -> Self {
        Self { kind, location }
    }

    #[allow(unused)]
    pub(super) fn kind(&self) -> &DefinitionParserErrorKind {
        &self.kind
    }

    #[allow(unused)]
    pub(super) fn location(&self) -> Location {
        self.location
    }
}

impl fmt::Display for DefinitionParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.location.line() == 0 {
            return match &self.kind {
                DefinitionParserErrorKind::NoRules => {
                    write!(f, "Lexer definition contains no valid rules.")
                }
                _ => write!(f, "Global error: {:?}", self.kind),
            };
        }

        write!(
            f,
            "Error at line {}, column {}: ",
            self.location.line(),
            self.location.column()
        )?;

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
    use crate::span::Location;

    #[test]
    fn accessors_return_constructor_data() {
        let err = DefinitionParserError::new(
            DefinitionParserErrorKind::MissingDelimiter,
            Location::new(3, 8),
        );

        assert!(matches!(
            err.kind(),
            DefinitionParserErrorKind::MissingDelimiter
        ));
        assert_eq!(err.location(), Location::new(3, 8));
    }
}
