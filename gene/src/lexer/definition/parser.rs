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

    /// Fetches next token, panics on lexer failure.
    /// (Trivia is now natively skipped by the Lexer runtime).
    fn advance(&mut self) -> Option<(Token<'a>, Range<usize>)> {
        let previous = self.peeked.take();

        match self.lexer.next() {
            Some(Ok(token)) => {
                self.peeked = Some((token, self.lexer.span()));
            }
            Some(Err(err)) => panic!("Fatal Lexer Error at byte offset {}", err.byte_offset),
            None => {
                self.peeked = None;
            }
        }

        previous
    }

    /// Helper to dynamically strip raw string boundaries (r#"..."# or c#"..."#)
    fn strip_raw_boundaries(slice: &'a str) -> &'a str {
        let quote_idx = slice.find('"').expect("Raw string must contain a quote");
        let prefix_len = quote_idx + 1;
        let suffix_len = quote_idx; // e.g., r#" is 3 bytes, "# is 2 bytes. (3 - 1 = 2)
        &slice[prefix_len..slice.len() - suffix_len]
    }

    fn parse_let_decl(&mut self) {
        let ident_str = match self.advance().unwrap().0 {
            Token::Identifier(name) => name,
            found => panic!("Expected Identifier, found {:?}", found),
        };

        match self.advance().unwrap().0 {
            Token::Assign(_) => {}
            found => panic!("Expected Assign, found {:?}", found),
        }

        let regex_slice = match self.advance().unwrap().0 {
            Token::RawRegex(slice) => slice,
            found => panic!("Expected RawRegex, found {:?}", found),
        };

        match self.advance().unwrap().0 {
            Token::Semi(_) => {}
            found => panic!("Expected Semi, found {:?}", found),
        }

        let inner_regex = Self::strip_raw_boundaries(regex_slice);
        let ast = RegexParser::new(inner_regex, 0).parse().unwrap();

        self.fragments.insert(
            ident_str,
            FragmentDecl {
                name: ident_str,
                ast,
                span: 0..0, // Set up spans later if diagnostic tracking is required
            },
        );
    }

    fn parse_rule_decl(&mut self, skip: bool) {
        let ident_str = match self.advance().unwrap().0 {
            Token::Identifier(name) => name,
            found => panic!("Expected Identifier, found {:?}", found),
        };

        match self.advance().unwrap().0 {
            Token::Assign(_) => {}
            found => panic!("Expected Assign, found {:?}", found),
        }

        let regex_slice = match self.advance().unwrap().0 {
            Token::RawRegex(slice) => slice,
            found => panic!("Expected RawRegex, found {:?}", found),
        };

        // Optional Code Block Lookahead
        let action = if let Some((peeked_token, _)) = &self.peeked {
            if let Token::CodeBlock(slice) = peeked_token {
                let code_slice = *slice;
                self.advance(); // Consume the code block token
                Some(Self::strip_raw_boundaries(code_slice))
            } else {
                None
            }
        } else {
            None
        };

        match self.advance().unwrap().0 {
            Token::Semi(_) => {}
            found => panic!("Expected Semi, found {:?}", found),
        }

        let inner_regex = Self::strip_raw_boundaries(regex_slice);
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
                Token::LetKw(_) => {
                    self.advance();
                    self.parse_let_decl();
                }
                Token::RuleKw(_) => {
                    self.advance();
                    self.parse_rule_decl(false);
                }
                Token::SkipKw(_) => {
                    self.advance();
                    match self.advance().unwrap().0 {
                        Token::RuleKw(_) => {}
                        found => panic!("Expected RuleKw after SkipKw, found {:?}", found),
                    }
                    self.parse_rule_decl(true);
                }
                _ => panic!(
                    "Expected 'let', 'rule', or 'skip', found {:?}",
                    peeked_token
                ),
            }
        }

        LexerDef {
            fragments: self.fragments,
            rules: self.rules,
        }
    }
}
