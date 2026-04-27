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
                kind: RegexParserErrorKind::UnescapedClassMetaChar(c),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(input: &str) -> RegexAst {
        RegexParser::new(input, 0)
            .parse()
            .expect("regex should parse")
    }

    fn parse_err(input: &str) -> RegexParserError {
        RegexParser::new(input, 0)
            .parse()
            .expect_err("regex should fail")
    }

    fn literal(ast: &RegexAst) -> char {
        match ast {
            RegexAst::Literal(c) => *c,
            other => panic!("expected literal, got {other:?}"),
        }
    }

    #[test]
    fn parses_single_literal() {
        assert_eq!(literal(&parse_ok("a")), 'a');
    }

    #[test]
    fn parses_unicode_literal() {
        assert_eq!(literal(&parse_ok("é")), 'é');
    }

    #[test]
    fn parses_wildcard() {
        assert!(matches!(parse_ok("."), RegexAst::Wildcard));
    }

    #[test]
    fn parses_basic_escapes() {
        assert_eq!(literal(&parse_ok("\\n")), '\n');
        assert_eq!(literal(&parse_ok("\\t")), '\t');
        assert_eq!(literal(&parse_ok("\\r")), '\r');
    }

    #[test]
    fn parses_escaped_meta_chars_as_literals() {
        for pattern in [
            "\\*", "\\+", "\\?", "\\|", "\\(", "\\)", "\\[", "\\]", "\\.", "\\\\",
        ] {
            assert!(matches!(parse_ok(pattern), RegexAst::Literal(_)));
        }
    }

    #[test]
    fn alternation_is_left_associative() {
        let ast = parse_ok("a|b|c");
        match ast {
            RegexAst::Alternation(left, right) => {
                assert_eq!(literal(&right), 'c');
                match *left {
                    RegexAst::Alternation(inner_left, inner_right) => {
                        assert_eq!(literal(&inner_left), 'a');
                        assert_eq!(literal(&inner_right), 'b');
                    }
                    other => panic!("expected nested alternation, got {other:?}"),
                }
            }
            other => panic!("expected alternation, got {other:?}"),
        }
    }

    #[test]
    fn concatenation_binds_tighter_than_alternation() {
        let ast = parse_ok("ab|c");
        match ast {
            RegexAst::Alternation(left, right) => {
                assert_eq!(literal(&right), 'c');
                match *left {
                    RegexAst::Concatenation(inner_left, inner_right) => {
                        assert_eq!(literal(&inner_left), 'a');
                        assert_eq!(literal(&inner_right), 'b');
                    }
                    other => panic!("expected concatenation, got {other:?}"),
                }
            }
            other => panic!("expected alternation, got {other:?}"),
        }
    }

    #[test]
    fn parses_closure_operators() {
        assert!(matches!(parse_ok("a*"), RegexAst::KleeneClosure(_)));
        assert!(matches!(parse_ok("a+"), RegexAst::PositiveClosure(_)));
        assert!(matches!(parse_ok("a?"), RegexAst::OptionalClosure(_)));
    }

    #[test]
    fn grouped_expression_parses() {
        let ast = parse_ok("(ab)c");
        match ast {
            RegexAst::Concatenation(left, right) => {
                assert!(matches!(*left, RegexAst::Concatenation(_, _)));
                assert_eq!(literal(&right), 'c');
            }
            other => panic!("expected concatenation, got {other:?}"),
        }
    }

    #[test]
    fn parses_character_class_and_range() {
        let ast = parse_ok("[a-cx]");
        match ast {
            RegexAst::CharacterClass(set) => {
                assert_eq!(set.ranges().len(), 2);
                assert_eq!(set.ranges()[0], 'a'..='c');
                assert_eq!(set.ranges()[1], 'x'..='x');
            }
            other => panic!("expected class, got {other:?}"),
        }
    }

    #[test]
    fn parses_negated_character_class() {
        assert!(matches!(parse_ok("[^a-z]"), RegexAst::ComplementClass(_)));
    }

    #[test]
    fn escaped_right_bracket_works_globally() {
        assert_eq!(literal(&parse_ok("\\]")), ']');
    }

    #[test]
    fn unescaped_right_bracket_fails_globally() {
        let err = parse_err("]");
        assert_eq!(err.kind, RegexParserErrorKind::UnescapedMetaChar(']'));
        assert_eq!(err.span, 0..1);
    }

    #[test]
    fn escaped_right_bracket_works_in_class() {
        let ast = parse_ok("[\\]]");
        match ast {
            RegexAst::CharacterClass(set) => {
                assert_eq!(set.ranges().len(), 1);
                assert_eq!(set.ranges()[0], ']'..=']');
            }
            other => panic!("expected class, got {other:?}"),
        }
    }

    #[test]
    fn escaped_hyphen_works_in_class() {
        let ast = parse_ok("[\\-]");
        match ast {
            RegexAst::CharacterClass(set) => {
                assert_eq!(set.ranges().len(), 1);
                assert_eq!(set.ranges()[0], '-'..='-');
            }
            other => panic!("expected class, got {other:?}"),
        }
    }

    #[test]
    fn unescaped_hyphen_fails_at_class_start() {
        let err = parse_err("[-a]");
        assert_eq!(err.kind, RegexParserErrorKind::UnescapedClassMetaChar('-'));
        assert_eq!(err.span, 1..2);
    }

    #[test]
    fn unescaped_hyphen_fails_at_class_end() {
        let err = parse_err("[a-]");
        assert_eq!(err.kind, RegexParserErrorKind::UnescapedClassMetaChar(']'));
        assert_eq!(err.span, 3..4);
    }

    #[test]
    fn unescaped_right_bracket_cannot_be_class_data() {
        let err = parse_err("[a-]]");
        assert_eq!(err.kind, RegexParserErrorKind::UnescapedClassMetaChar(']'));
        assert_eq!(err.span, 3..4);
    }

    #[test]
    fn invalid_reversed_range_reports_error() {
        let err = parse_err("[z-a]");
        assert_eq!(
            err.kind,
            RegexParserErrorKind::InvalidCharRange {
                start: 'z',
                end: 'a',
            }
        );
    }

    #[test]
    fn trailing_backslash_reports_error() {
        let err = parse_err("\\");
        assert_eq!(err.kind, RegexParserErrorKind::TrailingBackslash);
        assert_eq!(err.span, 0..1);
    }

    #[test]
    fn trailing_backslash_in_class_reports_error() {
        let err = parse_err("[a\\");
        assert_eq!(err.kind, RegexParserErrorKind::TrailingBackslash);
        assert_eq!(err.span, 2..3);
    }

    #[test]
    fn unclosed_group_reports_error() {
        let err = parse_err("(ab");
        assert_eq!(err.kind, RegexParserErrorKind::UnclosedGrouping);
    }

    #[test]
    fn unclosed_class_reports_error() {
        let err = parse_err("[ab");
        assert_eq!(err.kind, RegexParserErrorKind::UnclosedCharClass);
    }

    #[test]
    fn unescaped_operator_reports_error() {
        let err = parse_err("*");
        assert_eq!(err.kind, RegexParserErrorKind::UnescapedMetaChar('*'));
        assert_eq!(err.span, 0..1);
    }

    #[test]
    fn unexpected_trailing_chars_reports_error() {
        let err = parse_err("a)");
        assert_eq!(err.kind, RegexParserErrorKind::UnexpectedTrailingChars);
        assert_eq!(err.span, 1..2);
    }

    #[test]
    fn offsets_are_applied_to_error_spans() {
        let err = RegexParser::new("]", 10)
            .parse()
            .expect_err("regex should fail");
        assert_eq!(err.kind, RegexParserErrorKind::UnescapedMetaChar(']'));
        assert_eq!(err.span, 10..11);
    }
}
