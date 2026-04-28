use std::collections::HashMap;
use std::ops::Range;

use crate::lexer::{
    definition::{
        core::{FragmentDecl, LexerDef, TokenRule},
        lexer::{Lexer, Token},
    },
    regex::RegexParser,
};

pub(crate) struct DefinitionParser<'a> {
    lexer: Lexer<'a>,
    peeked: Option<(Token<'a>, Range<usize>)>,
    fragments: HashMap<&'a str, FragmentDecl<'a>>,
    rules: Vec<TokenRule<'a>>,
}

impl<'a> DefinitionParser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(input),
            peeked: None,
            fragments: HashMap::new(),
            rules: Vec::new(),
        };
        parser.advance();
        parser
    }

    /// Fetches next token, defensively discarding trivia, panics on lexer failure.
    fn advance(&mut self) -> Option<(Token<'a>, Range<usize>)> {
        let previous = self.peeked.take();

        loop {
            match self.lexer.next() {
                Some(Ok(token)) => {
                    if matches!(token, Token::Whitespace | Token::Comment) {
                        continue;
                    }
                    self.peeked = Some((token, self.lexer.span()));
                    break;
                }
                Some(Err(err)) => panic!("Fatal Lexer Error at byte offset {}", err.byte_offset),
                None => {
                    self.peeked = None;
                    break;
                }
            }
        }

        previous
    }

    fn expect(&mut self, expected: Token<'a>) -> (Token<'a>, Range<usize>) {
        let spanned = self.advance().expect("Unexpected EOF");
        if spanned.0 != expected {
            panic!("Expected {:?}, found {:?}", expected, spanned.0);
        }
        spanned
    }

    fn parse_let_decl(&mut self) {
        let ident_str = match self.advance().unwrap().0 {
            Token::Identifier(name) => name,
            found => panic!("Expected Identifier, found {:?}", found),
        };

        self.expect(Token::Assign);

        let regex_slice = match self.advance().unwrap().0 {
            Token::RawRegex(slice) => slice,
            found => panic!("Expected RawRegex, found {:?}", found),
        };

        self.expect(Token::Semi);

        // Strip boundaries: r#` and `#
        let inner_regex = &regex_slice[3..regex_slice.len() - 2];
        let ast = RegexParser::new(inner_regex, 0).parse().unwrap();

        self.fragments.insert(
            ident_str,
            FragmentDecl {
                name: ident_str,
                ast,
                span: 0..0,
            },
        );
    }

    fn parse_rule_decl(&mut self, skip: bool) {
        let ident_str = match self.advance().unwrap().0 {
            Token::Identifier(name) => name,
            found => panic!("Expected Identifier, found {:?}", found),
        };

        self.expect(Token::Assign);

        let regex_slice = match self.advance().unwrap().0 {
            Token::RawRegex(slice) => slice,
            found => panic!("Expected RawRegex, found {:?}", found),
        };

        // Optional Code Block Lookahead
        let action = if let Some((peeked_token, _)) = &self.peeked {
            if matches!(peeked_token, Token::CodeBlock(_)) {
                let code_slice = match self.advance().unwrap().0 {
                    Token::CodeBlock(slice) => slice,
                    _ => unreachable!(),
                };
                // Strip boundaries: c#` and `#
                Some(&code_slice[3..code_slice.len() - 2])
            } else {
                None
            }
        } else {
            None
        };

        self.expect(Token::Semi);

        let inner_regex = &regex_slice[3..regex_slice.len() - 2];
        let ast = RegexParser::new(inner_regex, 0).parse().unwrap();

        self.rules.push(TokenRule {
            name: ident_str,
            ast,
            action,
            skip,
        });
    }

    pub(crate) fn parse(mut self) -> LexerDef<'a> {
        while let Some((peeked_token, _)) = &self.peeked {
            match peeked_token {
                Token::LetKw => {
                    self.advance();
                    self.parse_let_decl();
                }
                Token::RuleKw => {
                    self.advance();
                    self.parse_rule_decl(false);
                }
                Token::SkipKw => {
                    self.advance();
                    self.expect(Token::RuleKw);
                    self.parse_rule_decl(true);
                }
                _ => panic!("Expected 'let' or 'rule', found {:?}", peeked_token),
            }
        }

        LexerDef {
            fragments: self.fragments,
            rules: self.rules,
        }
    }
}
