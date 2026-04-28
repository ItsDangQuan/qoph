use std::collections::HashMap;

use crate::lexer::{
    definition::{
        core::{FragmentDecl, LexerDef, TokenRule},
        lexer::{Lexer, Spanned, Token},
    },
    regex::RegexParser,
};

pub(crate) struct DefinitionParser<'a> {
    lexer: Lexer<'a>,
    peeked: Option<Spanned<Token<'a>>>,
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

    /// Fetches next token, discarding trivia, panics on lexer failure.
    fn advance(&mut self) -> Option<Spanned<Token<'a>>> {
        let previous = self.peeked.take();

        loop {
            match self.lexer.next() {
                Some(Ok(spanned)) => {
                    if matches!(spanned.node, Token::Whitespace | Token::Comment) {
                        continue;
                    }
                    self.peeked = Some(spanned);
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

    fn expect(&mut self, expected: Token<'a>) -> Spanned<Token<'a>> {
        let spanned = self.advance().expect("Unexpected EOF");
        if spanned.node != expected {
            panic!("Expected {:?}, found {:?}", expected, spanned.node);
        }
        spanned
    }

    fn parse_let_decl(&mut self) {
        let ident_str = match self.advance().unwrap().node {
            Token::Identifier(name) => name,
            found => panic!("Expected Identifier, found {:?}", found),
        };

        self.expect(Token::Assign);

        let regex_slice = match self.advance().unwrap().node {
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

    fn parse_rule_decl(&mut self) {
        let ident_str = match self.advance().unwrap().node {
            Token::Identifier(name) => name,
            found => panic!("Expected Identifier, found {:?}", found),
        };

        self.expect(Token::Assign);

        let regex_slice = match self.advance().unwrap().node {
            Token::RawRegex(slice) => slice,
            found => panic!("Expected RawRegex, found {:?}", found),
        };

        // Optional Code Block Lookahead
        let action = if let Some(peeked) = &self.peeked {
            if matches!(peeked.node, Token::CodeBlock(_)) {
                let code_slice = match self.advance().unwrap().node {
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
        });
    }

    pub(crate) fn parse(mut self) -> LexerDef<'a> {
        while let Some(peeked) = &self.peeked {
            match peeked.node {
                Token::LetKw => {
                    self.advance();
                    self.parse_let_decl();
                }
                Token::RuleKw => {
                    self.advance();
                    self.parse_rule_decl();
                }
                _ => panic!("Expected 'let' or 'rule', found {:?}", peeked.node),
            }
        }

        LexerDef {
            fragments: self.fragments,
            rules: self.rules,
        }
    }
}
