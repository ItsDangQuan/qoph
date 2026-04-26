use std::{iter::Peekable, str::Chars};

use crate::span::{Location, Span};

use super::{
    CharSet, RegexAst, RegexAstKind, RegexParserError, RegexParserErrorKind, RegexParserResult,
};

/// Recursive-descent parser for the regex grammar used in lexer definitions.
pub struct RegexParser<'a> {
    chars: Peekable<Chars<'a>>,
    current_line: usize,
    current_column: usize,
}

impl<'a> RegexParser<'a> {
    /// Creates a parser over the supplied input.
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            current_line: 1,
            current_column: 1,
        }
    }

    /// Returns the parser's current 1-based source location.
    fn current_location(&self) -> Location {
        Location::new(self.current_line, self.current_column)
    }

    /// Consumes one character and advances the tracked location.
    fn consume(&mut self) -> Option<char> {
        match self.chars.next() {
            Some('\n') => {
                self.current_line += 1;
                self.current_column = 1;
                Some('\n')
            }
            Some(c) => {
                self.current_column += 1;
                Some(c)
            }
            None => None,
        }
    }

    /// Parses a full regular-expression expression.
    ///
    /// The parser accepts the complete grammar and rejects trailing input.
    pub fn parse(&mut self) -> RegexParserResult<RegexAst> {
        let ast = self.parse_alternation()?;
        if let Some(c) = self.chars.peek().copied() {
            let location = self.current_location();
            self.consume();
            return Err(RegexParserError::new(
                RegexParserErrorKind::UnexpectedChar(c),
                location,
            ));
        }
        Ok(ast)
    }

    /// Parses alternation, which is the lowest-precedence binary operator.
    fn parse_alternation(&mut self) -> RegexParserResult<RegexAst> {
        let start_loc = self.current_location();

        let mut left = self.parse_concatenation()?;

        while let Some(&'|') = self.chars.peek() {
            self.consume();
            let right = self.parse_concatenation()?;
            let end_loc = self.current_location();
            left = RegexAst::new(
                RegexAstKind::Alternation(Box::new(left), Box::new(right)),
                Span::new(start_loc, end_loc),
            )
        }

        Ok(left)
    }

    /// Parses one or more concatenated closures.
    fn parse_concatenation(&mut self) -> RegexParserResult<RegexAst> {
        let start_loc = self.current_location();

        let mut left = self.parse_closure()?;

        while let Some(&c) = self.chars.peek() {
            if matches!(c, '|' | ')' | '*' | '+' | '?') {
                break;
            }
            let right = self.parse_closure()?;
            let end_loc = self.current_location();
            left = RegexAst::new(
                RegexAstKind::Concatenation(Box::new(left), Box::new(right)),
                Span::new(start_loc, end_loc),
            )
        }

        Ok(left)
    }

    /// Parses a base expression followed by at most one postfix closure operator.
    fn parse_closure(&mut self) -> RegexParserResult<RegexAst> {
        let start_loc = self.current_location();

        let base = self.parse_base()?;

        if let Some(&c) = self.chars.peek() {
            match c {
                '*' => {
                    self.consume();
                    let end_loc = self.current_location();
                    return Ok(RegexAst::new(
                        RegexAstKind::KleeneClosure(Box::new(base)),
                        Span::new(start_loc, end_loc),
                    ));
                }
                '+' => {
                    self.consume();
                    let end_loc = self.current_location();
                    return Ok(RegexAst::new(
                        RegexAstKind::PositiveClosure(Box::new(base)),
                        Span::new(start_loc, end_loc),
                    ));
                }
                '?' => {
                    self.consume();
                    let end_loc = self.current_location();
                    return Ok(RegexAst::new(
                        RegexAstKind::OptionalClosure(Box::new(base)),
                        Span::new(start_loc, end_loc),
                    ));
                }
                _ => {}
            }
        }

        Ok(base)
    }

    /// Parses a base grammar production.
    fn parse_base(&mut self) -> RegexParserResult<RegexAst> {
        let start_loc = self.current_location();

        match self.consume() {
            Some('(') => {
                let expr = self.parse_alternation()?;
                if self.consume() != Some(')') {
                    return Err(RegexParserError::new(
                        RegexParserErrorKind::UnclosedGrouping,
                        self.current_location(),
                    ));
                }
                Ok(expr)
            }
            Some('[') => self.parse_character_class(),
            Some('\\') => {
                let c = self.parse_escape_sequence()?;
                let end_loc = self.current_location();
                Ok(RegexAst::new(
                    RegexAstKind::Literal(c),
                    Span::new(start_loc, end_loc),
                ))
            }
            Some('.') => {
                let end_loc = self.current_location();
                Ok(RegexAst::new(
                    RegexAstKind::Wildcard,
                    Span::new(start_loc, end_loc),
                ))
            }
            Some(c) => {
                let end_loc = self.current_location();
                Ok(RegexAst::new(
                    RegexAstKind::Literal(c),
                    Span::new(start_loc, end_loc),
                ))
            }
            None => Err(RegexParserError::new(
                RegexParserErrorKind::UnexpectedEof,
                self.current_location(),
            )),
        }
    }

    fn parse_character_class(&mut self) -> RegexParserResult<RegexAst> {
        let start_loc = self.current_location();

        let mut negated = false;

        if let Some(&'^') = self.chars.peek() {
            negated = true;
            self.consume();
        }

        let mut ranges = Vec::new();

        while let Some(&c) = self.chars.peek() {
            if c == ']' {
                self.consume();
                let end_loc = self.current_location();

                let charset = CharSet::new(ranges);
                return if negated {
                    Ok(RegexAst::new(
                        RegexAstKind::ComplementClass(charset),
                        Span::new(start_loc, end_loc),
                    ))
                } else {
                    Ok(RegexAst::new(
                        RegexAstKind::CharacterClass(charset),
                        Span::new(start_loc, end_loc),
                    ))
                };
            }

            let start_char = self.parse_class_item()?;

            if let Some(&'-') = self.chars.peek() {
                self.consume();

                if let Some(&']') = self.chars.peek() {
                    ranges.push(start_char..=start_char);
                    ranges.push('-'..='-');
                    continue;
                }

                let end_char = self.parse_class_item()?;

                if start_char > end_char {
                    return Err(RegexParserError::new(
                        RegexParserErrorKind::InvalidCharRange {
                            start: start_char,
                            end: end_char,
                        },
                        self.current_location(),
                    ));
                }
                ranges.push(start_char..=end_char);
            } else {
                ranges.push(start_char..=start_char);
            }
        }

        Err(RegexParserError::new(
            RegexParserErrorKind::UnclosedCharClass,
            self.current_location(),
        ))
    }

    /// Parses a single character-class item.
    fn parse_class_item(&mut self) -> RegexParserResult<char> {
        match self.consume() {
            Some('\\') => self.parse_escape_sequence(),
            Some(c) => Ok(c),
            None => Err(RegexParserError::new(
                RegexParserErrorKind::UnexpectedEof,
                self.current_location(),
            )),
        }
    }

    /// Parses an escape sequence and maps the supported escapes to characters.
    fn parse_escape_sequence(&mut self) -> RegexParserResult<char> {
        match self.consume() {
            Some('n') => Ok('\n'),
            Some('t') => Ok('\t'),
            Some('r') => Ok('\r'),
            Some(c) => Ok(c),
            None => Err(RegexParserError::new(
                RegexParserErrorKind::TrailingBackslash,
                self.current_location(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::span::Location;

    use super::*;

    fn parse_ok(input: &str) -> RegexAst {
        RegexParser::new(input).parse().expect("regex should parse")
    }

    fn parse_err(input: &str) -> RegexParserError {
        RegexParser::new(input)
            .parse()
            .expect_err("regex should fail")
    }

    fn literal(ast: &RegexAst) -> char {
        match ast.kind() {
            RegexAstKind::Literal(c) => *c,
            other => panic!("expected literal, got {other:?}"),
        }
    }

    #[test]
    fn parses_single_literal() {
        let ast = parse_ok("a");
        assert_eq!(literal(&ast), 'a');
        assert_eq!(ast.span().start(), Location::new(1, 1));
        assert_eq!(ast.span().end(), Location::new(1, 2));
    }

    #[test]
    fn parses_unicode_literal_as_single_char() {
        let ast = parse_ok("é");
        assert_eq!(literal(&ast), 'é');
        assert_eq!(ast.span().start(), Location::new(1, 1));
        assert_eq!(ast.span().end(), Location::new(1, 2));
    }

    #[test]
    fn parses_wildcard() {
        let ast = parse_ok(".");
        assert!(matches!(ast.kind(), RegexAstKind::Wildcard));
    }

    #[test]
    fn parses_escape_sequences() {
        assert_eq!(literal(&parse_ok("\\n")), '\n');
        assert_eq!(literal(&parse_ok("\\t")), '\t');
        assert_eq!(literal(&parse_ok("\\r")), '\r');
        assert_eq!(literal(&parse_ok("\\*")), '*');
    }

    #[test]
    fn alternation_is_left_associative() {
        let ast = parse_ok("a|b|c");

        match ast.kind() {
            RegexAstKind::Alternation(left, right) => {
                assert_eq!(literal(right), 'c');
                match left.kind() {
                    RegexAstKind::Alternation(inner_left, inner_right) => {
                        assert_eq!(literal(inner_left), 'a');
                        assert_eq!(literal(inner_right), 'b');
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

        match ast.kind() {
            RegexAstKind::Alternation(left, right) => {
                assert_eq!(literal(right), 'c');
                match left.kind() {
                    RegexAstKind::Concatenation(inner_left, inner_right) => {
                        assert_eq!(literal(inner_left), 'a');
                        assert_eq!(literal(inner_right), 'b');
                    }
                    other => panic!("expected concatenation on left branch, got {other:?}"),
                }
            }
            other => panic!("expected alternation, got {other:?}"),
        }
    }

    #[test]
    fn parses_concatenation() {
        let ast = parse_ok("(a)b");

        match ast.kind() {
            RegexAstKind::Concatenation(left, right) => {
                assert_eq!(literal(left), 'a');
                assert_eq!(literal(right), 'b');
            }
            other => panic!("expected concatenation, got {other:?}"),
        }

        assert_eq!(ast.span().start(), Location::new(1, 1));
        assert_eq!(ast.span().end(), Location::new(1, 5));
    }

    #[test]
    fn parses_closure_operators() {
        assert!(matches!(
            parse_ok("a*").kind(),
            RegexAstKind::KleeneClosure(_)
        ));
        assert!(matches!(
            parse_ok("a+").kind(),
            RegexAstKind::PositiveClosure(_)
        ));
        assert!(matches!(
            parse_ok("a?").kind(),
            RegexAstKind::OptionalClosure(_)
        ));
    }

    #[test]
    fn parses_grouped_closure() {
        let ast = parse_ok("(ab)*");
        assert!(matches!(ast.kind(), RegexAstKind::KleeneClosure(_)));
    }

    #[test]
    fn parses_character_class_single_and_range() {
        let ast = parse_ok("[a-cx]");

        match ast.kind() {
            RegexAstKind::CharacterClass(charset) => {
                assert_eq!(charset.ranges().len(), 2);
                assert_eq!(charset.ranges()[0], 'a'..='c');
                assert_eq!(charset.ranges()[1], 'x'..='x');
            }
            other => panic!("expected character class, got {other:?}"),
        }
    }

    #[test]
    fn merges_character_class_ranges() {
        let ast = parse_ok("[a-cb-d]");

        match ast.kind() {
            RegexAstKind::CharacterClass(charset) => {
                assert_eq!(charset.ranges().len(), 1);
                assert_eq!(charset.ranges()[0], 'a'..='d');
            }
            other => panic!("expected character class, got {other:?}"),
        }
    }

    #[test]
    fn parses_negated_character_class() {
        let ast = parse_ok("[^a-z]");
        assert!(matches!(ast.kind(), RegexAstKind::ComplementClass(_)));
    }

    #[test]
    fn parses_hyphen_literal_at_class_end() {
        let ast = parse_ok("[a-]");
        match ast.kind() {
            RegexAstKind::CharacterClass(charset) => {
                assert!(charset.ranges().contains(&('a'..='a')));
                assert!(charset.ranges().contains(&('-'..='-')));
            }
            other => panic!("expected character class, got {other:?}"),
        }
    }

    #[test]
    fn parses_escaped_closing_bracket_in_class() {
        let ast = parse_ok("[\\]]");
        match ast.kind() {
            RegexAstKind::CharacterClass(charset) => {
                assert_eq!(charset.ranges().len(), 1);
                assert_eq!(charset.ranges()[0], ']'..=']');
            }
            other => panic!("expected character class, got {other:?}"),
        }
    }

    #[test]
    fn reports_unclosed_character_class() {
        let error = parse_err("[ab");
        assert_eq!(error.kind(), &RegexParserErrorKind::UnclosedCharClass);
        assert_eq!(error.location(), Location::new(1, 4));
    }

    #[test]
    fn reports_invalid_character_range() {
        let error = parse_err("[z-a]");
        assert_eq!(
            error.kind(),
            &RegexParserErrorKind::InvalidCharRange {
                start: 'z',
                end: 'a'
            }
        );
    }

    #[test]
    fn reports_trailing_backslash() {
        let error = parse_err("\\");
        assert_eq!(error.kind(), &RegexParserErrorKind::TrailingBackslash);
        assert_eq!(error.location(), Location::new(1, 2));
    }

    #[test]
    fn reports_unclosed_grouping() {
        let error = parse_err("(a");
        assert_eq!(error.kind(), &RegexParserErrorKind::UnclosedGrouping);
        assert_eq!(error.location(), Location::new(1, 3));
    }

    #[test]
    fn reports_unclosed_grouping_with_newline_location() {
        let error = parse_err("(\n");
        assert_eq!(error.kind(), &RegexParserErrorKind::UnclosedGrouping);
        assert_eq!(error.location(), Location::new(2, 1));
    }

    #[test]
    fn reports_unexpected_trailing_char() {
        let error = parse_err("a)");
        assert_eq!(error.kind(), &RegexParserErrorKind::UnexpectedChar(')'));
        assert_eq!(error.location(), Location::new(1, 2));
    }
}
