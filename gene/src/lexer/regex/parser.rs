use std::ops::RangeInclusive;

use super::char_set::CharSet;
use super::core::RegexAst;
use super::error::{RegexParserError, RegexParserErrorKind};

#[derive(Debug)]
/// Recursive-descent parser for the regex grammar used in lexer definitions.
pub(crate) struct RegexParser<'a> {
    input: &'a str,
    cursor: usize,
    offset: usize,
}

impl<'a> RegexParser<'a> {
    /// Instantiates a new parser tied to an absolute file offset.
    pub(crate) fn new(input: &'a str, offset: usize) -> Self {
        Self {
            input,
            cursor: 0,
            offset,
        }
    }

    /// Parses a full regular-expression expression, consuming the parser.
    pub(crate) fn parse(mut self) -> Result<RegexAst, RegexParserError> {
        let ast = self.parse_alternation()?;

        if let Some(c) = self.peek() {
            let loc = self.current_location();
            return Err(RegexParserError {
                kind: RegexParserErrorKind::UnexpectedTrailingChars,
                span: loc..loc + c.len_utf8(),
            });
        }

        Ok(ast)
    }

    /// Calculates the current absolute file coordinate.
    fn current_location(&self) -> usize {
        self.offset + self.cursor
    }

    /// Peek at the next token without advancing the internal state.
    fn peek(&self) -> Option<char> {
        self.input[self.cursor..].chars().next()
    }

    /// Return the next token and advances the internal state.
    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.cursor += c.len_utf8();
        Some(c)
    }

    fn parse_alternation(&mut self) -> Result<RegexAst, RegexParserError> {
        let mut left = self.parse_concatenation()?;

        while let Some('|') = self.peek() {
            self.next();
            let right = self.parse_concatenation()?;
            left = RegexAst::Alternation(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_concatenation(&mut self) -> Result<RegexAst, RegexParserError> {
        let mut left = self.parse_closure()?;

        while let Some(c) = self.peek()
            && !matches!(c, '|' | ')')
        {
            let right = self.parse_closure()?;
            left = RegexAst::Concatenation(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_closure(&mut self) -> Result<RegexAst, RegexParserError> {
        let ast = self.parse_base()?;

        match self.peek() {
            Some('*') => {
                self.next();
                Ok(RegexAst::KleeneClosure(Box::new(ast)))
            }
            Some('+') => {
                self.next();
                Ok(RegexAst::PositiveClosure(Box::new(ast)))
            }
            Some('?') => {
                self.next();
                Ok(RegexAst::OptionalClosure(Box::new(ast)))
            }
            _ => Ok(ast),
        }
    }

    fn parse_base(&mut self) -> Result<RegexAst, RegexParserError> {
        match self.peek() {
            Some('(') => {
                let loc = self.current_location();
                self.next();
                let ast = self.parse_alternation()?;
                match self.next() {
                    Some(')') => Ok(ast),
                    _ => Err(RegexParserError {
                        kind: RegexParserErrorKind::UnclosedGrouping,
                        span: loc..self.current_location(),
                    }),
                }
            }
            Some('[') => {
                let loc = self.current_location();
                self.next();
                let ast = self.parse_character_class()?;
                match self.next() {
                    Some(']') => Ok(ast),
                    _ => Err(RegexParserError {
                        kind: RegexParserErrorKind::UnclosedCharClass,
                        span: loc..self.current_location(),
                    }),
                }
            }
            Some('\\') => {
                let loc = self.current_location();
                self.next();
                match self.parse_escape_code() {
                    Some(c) => Ok(RegexAst::Literal(c)),
                    None => Err(RegexParserError {
                        kind: RegexParserErrorKind::TrailingBackslash,
                        span: loc..self.current_location(),
                    }),
                }
            }
            Some('.') => {
                self.next();
                Ok(RegexAst::Wildcard)
            }
            Some(c) if matches!(c, '|' | '*' | '+' | '?' | '(' | ')' | '[' | ']') => {
                let loc = self.current_location();
                self.next();
                Err(RegexParserError {
                    kind: RegexParserErrorKind::UnescapedMetaChar(c),
                    span: loc..self.current_location(),
                })
            }
            Some(c) => {
                self.next();
                Ok(RegexAst::Literal(c))
            }
            None => {
                let loc = self.current_location();
                Err(RegexParserError {
                    kind: RegexParserErrorKind::UnexpectedEof,
                    span: loc..self.current_location(),
                })
            }
        }
    }

    fn parse_escape_code(&mut self) -> Option<char> {
        match self.next() {
            Some('n') => Some('\n'),
            Some('t') => Some('\t'),
            Some('r') => Some('\r'),
            opt => opt,
        }
    }

    fn parse_character_class(&mut self) -> Result<RegexAst, RegexParserError> {
        let negated = match self.peek() {
            Some('^') => {
                self.next();
                true
            }
            _ => false,
        };

        let mut ranges = Vec::new();
        while let Some(c) = self.peek()
            && !matches!(c, ']')
        {
            let range = self.parse_class_item()?;
            ranges.push(range);
        }

        let char_set = CharSet::new(ranges);
        match negated {
            true => Ok(RegexAst::ComplementClass(char_set)),
            false => Ok(RegexAst::CharacterClass(char_set)),
        }
    }

    fn parse_class_item(&mut self) -> Result<RangeInclusive<char>, RegexParserError> {
        let loc = self.current_location();
        let left = self.parse_class_bound()?;

        match self.peek() {
            Some('-') => {
                self.next();
                let right = self.parse_class_bound()?;

                if left > right {
                    return Err(RegexParserError {
                        kind: RegexParserErrorKind::InvalidCharRange {
                            start: left,
                            end: right,
                        },
                        span: loc..self.current_location(),
                    });
                }

                Ok(left..=right)
            }
            _ => Ok(left..=left),
        }
    }

    fn parse_class_bound(&mut self) -> Result<char, RegexParserError> {
        let loc = self.current_location();
        match self.next() {
            Some('\\') => self.parse_escape_code().ok_or(RegexParserError {
                kind: RegexParserErrorKind::TrailingBackslash,
                span: loc..self.current_location(),
            }),
            Some(c) if matches!(c, ']' | '-') => Err(RegexParserError {
                kind: RegexParserErrorKind::UnescapedMetaChar(c),
                span: loc..self.current_location(),
            }),
            Some(c) => Ok(c),
            None => Err(RegexParserError {
                kind: RegexParserErrorKind::UnexpectedEof,
                span: loc..self.current_location(),
            }),
        }
    }
}
