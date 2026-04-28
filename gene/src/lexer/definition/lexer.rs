#[rustfmt::skip]
#[allow(dead_code)]
mod generated {

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    LetKw(&'a str),
    RuleKw(&'a str),
    SkipKw(&'a str),
    Assign(&'a str),
    Semi(&'a str),
    Identifier(&'a str),
    RawRegex(&'a str),
    CodeBlock(&'a str),
}

const CLASS_MAP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    1, 0, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 6, 0, 7, 0, 0, 
    0, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 8, 
    0, 8, 8, 9, 8, 10, 8, 8, 8, 11, 8, 12, 13, 8, 8, 8, 
    14, 8, 15, 16, 17, 18, 8, 8, 8, 8, 8, 0, 0, 0, 0, 0, 
    19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 
    22, 22, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 
    24, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 26, 25, 25, 
    27, 28, 28, 28, 29, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22,
];

const TRANSITIONS: [[u16; 30]; 30] = [
    [0; 30],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 17, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 19, 16, 17, 15, 1, 26, 14, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 20, 23, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 11, 12, 23, 0, 0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 19, 19, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 2, 2, 0, 3, 0, 8, 7, 23, 13, 23, 23, 23, 9, 23, 25, 29, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 4, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 5, 23, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 23, 23, 6, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 10, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 10, 24, 23, 0, 0, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 17, 17, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 23, 21, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 22, 23, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 23, 0, 0, 23, 23, 23, 23, 28, 23, 23, 23, 23, 23, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
];

const ACCEPT: [Option<usize>; 30] = [
    None,
    None,
    Some(0),
    Some(1),
    Some(2),
    Some(3),
    Some(4),
    Some(5),
    Some(6),
    Some(7),
    Some(8),
    Some(9),
    None,
    Some(7),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(7),
    Some(7),
    Some(7),
    Some(7),
    None,
    Some(7),
    None,
    Some(7),
    Some(7),
    Some(7),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub byte_offset: usize,
}

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    token_start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: 0,
            token_start: 0,
        }
    }

    pub fn span(&self) -> Range<usize> {
        self.token_start..self.cursor
    }

    pub fn slice(&self) -> &'a str {
        &self.input[self.token_start..self.cursor]
    }

    pub fn spanned(self) -> SpannedIter<'a> {
        SpannedIter { lexer: self }
    }
}

pub struct SpannedIter<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Iterator for SpannedIter<'a> {
    type Item = (Result<Token<'a>, LexerError>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let token_res = self.lexer.next()?;
        let span = self.lexer.span();
        Some((token_res, span))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.input.len() {
            return None;
        }

        let bytes = self.input.as_bytes();
        self.token_start = self.cursor;

        let mut current_state: u16 = 18;
        let mut last_accept_state = None;
        let mut last_accept_cursor = self.cursor;

        while self.cursor < bytes.len() {
            let byte = bytes[self.cursor];
            let class_id = CLASS_MAP[byte as usize];
            let next_state = TRANSITIONS[current_state as usize][class_id as usize];

            if next_state == 0 {
                break;
            }

            current_state = next_state;
            self.cursor += 1;

            if let Some(rule_id) = ACCEPT[current_state as usize] {
                last_accept_state = Some(rule_id);
                last_accept_cursor = self.cursor;
            }
        }

        if let Some(rule_id) = last_accept_state {
            self.cursor = last_accept_cursor;
            let slice = &self.input[self.token_start..self.cursor];

            let token = match rule_id {
                0 => { return self.next(); },
                1 => { return self.next(); },
                2 => Token::LetKw(slice),
                3 => Token::RuleKw(slice),
                4 => Token::SkipKw(slice),
                5 => Token::Assign(slice),
                6 => Token::Semi(slice),
                7 => Token::Identifier(slice),
                8 => {
                    let hash_count = slice.len() - 2; 
                    let mut search_offset = self.cursor;
                    let final_token;

                    loop {
                        let window = &self.input[search_offset..];
                        match window.find('"') {
                            Some(quote_rel_idx) => {
                                let quote_abs_idx = search_offset + quote_rel_idx;
                                let hash_start = quote_abs_idx + 1;
                                let hash_end = hash_start + hash_count;

                                if hash_end <= self.input.len() && self.input[hash_start..hash_end].bytes().all(|b| b == b'#') {
                                    self.cursor = hash_end;
                                    final_token = Token::RawRegex(&self.input[self.token_start..self.cursor]);
                                    break;
                                }
                                search_offset = quote_abs_idx + 1;
                            }
                            None => {
                                self.cursor = self.input.len();
                                return Some(Err(LexerError { byte_offset: self.token_start }));
                            }
                        }
                    }
                    final_token
                },
                9 => {
                    let hash_count = slice.len() - 2; 
                    let mut search_offset = self.cursor;
                    let final_token;

                    loop {
                        let window = &self.input[search_offset..];
                        match window.find('"') {
                            Some(quote_rel_idx) => {
                                let quote_abs_idx = search_offset + quote_rel_idx;
                                let hash_start = quote_abs_idx + 1;
                                let hash_end = hash_start + hash_count;

                                if hash_end <= self.input.len() && self.input[hash_start..hash_end].bytes().all(|b| b == b'#') {
                                    self.cursor = hash_end;
                                    final_token = Token::CodeBlock(&self.input[self.token_start..self.cursor]);
                                    break;
                                }
                                search_offset = quote_abs_idx + 1;
                            }
                            None => {
                                self.cursor = self.input.len();
                                return Some(Err(LexerError { byte_offset: self.token_start }));
                            }
                        }
                    }
                    final_token
                },
                _ => unreachable!(),
            };

            Some(Ok(token))
        } else {
            let next_char = self.input[self.token_start..].chars().next().unwrap();
            self.cursor = self.token_start + next_char.len_utf8();
            Some(Err(LexerError { byte_offset: self.token_start }))
        }
    }
}

}

#[allow(unused_imports)]
pub use generated::{Lexer, LexerError, SpannedIter, Token};
