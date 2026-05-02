use std::ops::Range;

use crate::lexer::regex::RegexParser;
use crate::lexer::{LexerCompilerDiag, LexerCompilerError};

use super::core::{FragmentRule, Rule, TokenRule};
use super::lexer::{Lexer, SpannedIter, Token};

pub(crate) struct RuleParser<'a> {
    lexer: SpannedIter<'a>,
    peeked: Option<(Token, Range<usize>)>,
    fragments: Vec<FragmentRule<'a>>,
    rules: Vec<TokenRule<'a>>,
    state_type: Option<&'a str>,
    pub(crate) diag: Vec<LexerCompilerDiag>,
}

impl<'a> RuleParser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(input).spanned(),
            peeked: None,
            fragments: Vec::new(),
            rules: Vec::new(),
            state_type: None,
            diag: Vec::new(),
        };
        parser.advance();
        parser
    }

    /// Fetches next token and accumulates lexing errors.
    fn advance(&mut self) {
        loop {
            match self.lexer.next() {
                Some((Ok(token), span)) => {
                    self.peeked = Some((token, span));
                    break;
                }
                Some((Err(()), span)) => self.diag.push(LexerCompilerDiag::new(
                    LexerCompilerError::UnrecognizedToken,
                    span,
                )),
                None => {
                    self.peeked = None;
                    break;
                }
            }
        }
    }

    fn peek_span(&self) -> Range<usize> {
        self.peeked
            .as_ref()
            .map(|(_, span)| span.clone())
            .unwrap_or_else(|| self.lexer.span())
    }

    fn report_unexpected(&mut self, expected: &str) {
        if let Some((_, span)) = &self.peeked {
            self.diag.push(LexerCompilerDiag::new(
                LexerCompilerError::UnexpectedToken {
                    expected: expected.to_string(),
                },
                span.clone(),
            ));
        } else {
            self.diag.push(LexerCompilerDiag::new(
                LexerCompilerError::UnexpectedEof {
                    expected: expected.to_string(),
                },
                self.lexer.span(),
            ))
        }
    }

    fn expect_ident(&mut self) -> Result<(&'a str, Range<usize>), ()> {
        if let Some((Token::Identifier, span)) = &self.peeked {
            let slice = &self.lexer.source()[span.clone()];
            let res = (slice, span.clone());
            self.advance();
            Ok(res)
        } else {
            self.report_unexpected("identifier");
            Err(())
        }
    }

    fn expect_path(&mut self) -> Result<(&'a str, Range<usize>), ()> {
        if let Some((Token::Path, span)) = &self.peeked {
            let slice = &self.lexer.source()[span.clone()];
            let res = (slice, span.clone());
            self.advance();
            Ok(res)
        } else {
            self.report_unexpected("absolute module path");
            Err(())
        }
    }

    fn expect_assign(&mut self) -> Result<Range<usize>, ()> {
        if let Some((Token::Assign, span)) = &self.peeked {
            let res = span.clone();
            self.advance();
            Ok(res)
        } else {
            self.report_unexpected("'='");
            Err(())
        }
    }

    fn expect_raw_regex(&mut self) -> Result<(&'a str, Range<usize>), ()> {
        if let Some((Token::RawRegex, span)) = &self.peeked {
            let slice = &self.lexer.source()[span.clone()];
            let res = (slice, span.clone());
            self.advance();
            Ok(res)
        } else {
            self.report_unexpected("regular expression");
            Err(())
        }
    }

    fn expect_semi(&mut self) -> Result<Range<usize>, ()> {
        if let Some((Token::Semi, span)) = &self.peeked {
            let res = span.clone();
            self.advance();
            Ok(res)
        } else {
            self.report_unexpected("';'");
            Err(())
        }
    }

    /// Helper to dynamically strip raw string boundaries (r#"..."# or c#"..."#)
    fn strip_raw_boundaries(slice: &'a str) -> (&'a str, usize) {
        let quote_idx = slice.find('"').expect("raw string must contain a quote");
        let prefix_len = quote_idx + 1;
        let suffix_len = quote_idx;
        (&slice[prefix_len..slice.len() - suffix_len], prefix_len)
    }

    fn parse_let_decl(&mut self) -> Result<(), ()> {
        let start_span = self.peek_span();
        self.advance();

        let (name, name_span) = self.expect_ident()?;
        self.expect_assign()?;
        let (regex_slice, regex_span) = self.expect_raw_regex()?;
        let end_span = self.expect_semi()?;

        let full_span = start_span.start..end_span.end;

        let (inner_regex, offset) = Self::strip_raw_boundaries(regex_slice);
        let absolute_start = regex_span.start + offset;

        match RegexParser::new(inner_regex, absolute_start).parse() {
            Ok(ast) => self.fragments.push(FragmentRule {
                name,
                name_span,
                ast,
                full_span,
            }),
            Err((error, span)) => self.diag.push(LexerCompilerDiag::new(error, span)),
        }

        Ok(())
    }

    fn parse_state_decl(&mut self) -> Result<(), ()> {
        self.advance();
        let (path_slice, _) = self.expect_path()?;
        self.expect_semi()?;
        self.state_type = Some(path_slice);
        Ok(())
    }

    fn parse_rule_decl(&mut self, skip: bool, start_span: Range<usize>) -> Result<(), ()> {
        self.advance();

        let (name, name_span) = self.expect_ident()?;
        self.expect_assign()?;
        let (regex_slice, regex_span) = self.expect_raw_regex()?;

        let mut callback = None;
        if let Some((Token::FatArrow, _)) = &self.peeked {
            self.advance();
            let (path_slice, _) = self.expect_path()?;
            callback = Some(path_slice);
        }

        let end_span = self.expect_semi()?;

        let full_span = start_span.start..end_span.end;

        let (inner_regex, offset) = Self::strip_raw_boundaries(regex_slice);
        let absolute_start = regex_span.start + offset;

        match RegexParser::new(inner_regex, absolute_start).parse() {
            Ok(ast) => self.rules.push(TokenRule {
                name,
                name_span,
                ast,
                callback,
                skip,
                full_span,
            }),
            Err((error, span)) => self.diag.push(LexerCompilerDiag::new(error, span)),
        }

        Ok(())
    }

    /// Discards tokens until the parser reaches a safe boundary to resume analysis.
    fn synchronize(&mut self) {
        while let Some((token, _)) = &self.peeked
            && !matches!(token, Token::LetKw | Token::RuleKw | Token::SkipKw)
        {
            self.advance()
        }
    }

    pub(crate) fn parse(mut self) -> (Rule<'a>, Vec<LexerCompilerDiag>) {
        if let Some((Token::StateKw, _)) = &self.peeked
            && self.parse_state_decl().is_err()
        {
            self.synchronize();
        }

        while let Some((peeked_token, _)) = &self.peeked {
            let res = match peeked_token {
                Token::LetKw => self.parse_let_decl(),
                Token::RuleKw => {
                    let start = self.peek_span();
                    self.parse_rule_decl(false, start)
                }
                Token::SkipKw => {
                    let start = self.peek_span();
                    self.advance();
                    if let Some((Token::RuleKw, _)) = &self.peeked {
                        self.parse_rule_decl(true, start)
                    } else {
                        self.report_unexpected("keyword 'rule'");
                        Err(())
                    }
                }
                _ => {
                    self.report_unexpected("keyword 'let', 'rule', or 'skip'");
                    Err(())
                }
            };

            if res.is_err() {
                self.synchronize();
            }
        }

        (
            Rule {
                fragments: self.fragments,
                rules: self.rules,
                state_type: self.state_type,
            },
            self.diag,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::RuleParser;

    #[test]
    fn parses_fragments_skip_rules_and_actions() {
        let input = r##"
let Digit = r"[0-9]";
skip rule Ws = r"[ \t\n\r]+";
rule Number = r"{Digit}+";
rule Ident = r"[a-zA-Z_][a-zA-Z0-9_]*" => crate::handle_ident;
    "##;

        let parser = RuleParser::new(input);
        let (rule, diag) = parser.parse();

        assert!(diag.is_empty());
        assert_eq!(rule.fragments.len(), 1);
        assert_eq!(rule.rules.len(), 3);

        assert!(rule.rules[0].skip);
        assert_eq!(rule.rules[1].name, "Number");
        assert_eq!(rule.rules[2].name, "Ident");
        assert_eq!(rule.rules[2].callback, Some("crate::handle_ident"));
    }
}
