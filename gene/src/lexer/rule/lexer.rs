#[rustfmt::skip]
#[allow(dead_code, unused)]
mod generated {

use std::collections::VecDeque;
use std::ops::{Deref, DerefMut, Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    StateKw,
    LetKw,
    RuleKw,
    SkipKw,
    Assign,
    Semi,
    FatArrow,
    Identifier,
    Path,
    RawRegex,
}

const CLASS_MAP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 1, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    1, 0, 3, 4, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 8, 0, 9, 10, 0, 
    0, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 0, 0, 0, 0, 11, 
    0, 12, 11, 11, 11, 13, 11, 11, 11, 14, 11, 15, 16, 11, 11, 11, 
    17, 11, 18, 19, 20, 21, 11, 11, 11, 11, 11, 0, 0, 0, 0, 0, 
    22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 
    25, 25, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 
    26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 
    27, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 29, 28, 28, 
    30, 31, 31, 31, 32, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
];

const TRANSITIONS: [[u16; 33]; 37] = [
    [0; 33],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 24, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 25, 23, 24, 22, 1, 33, 21, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 27, 30, 30, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 12, 17, 0, 0, 0, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 13, 14, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 2, 2, 0, 3, 19, 0, 0, 9, 8, 0, 30, 30, 30, 30, 30, 11, 30, 15, 36, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 25, 25, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 5, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 6, 30, 30, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 30, 7, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 24, 24, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 30, 28, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 29, 30, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 30, 17, 0, 0, 0, 30, 30, 30, 30, 35, 30, 30, 30, 30, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
];

const ACCEPT: [Option<usize>; 37] = [
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
    Some(10),
    Some(11),
    None,
    Some(9),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(9),
    Some(9),
    Some(9),
    Some(9),
    None,
    None,
    None,
    Some(9),
    Some(9),
    Some(9),
];

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    token_start: usize,
    buffer: VecDeque<Token>,
    pub state: (),
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: 0,
            token_start: 0,
            buffer: VecDeque::new(),
            state: (),
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

    pub fn source(&self) -> &'a str {
        self.input
    }
}

pub struct SpannedIter<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Iterator for SpannedIter<'a> {
    type Item = (Result<Token, ()>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let token_res = self.lexer.next()?;
        let span = self.lexer.span();
        Some((token_res, span))
    }
}

impl<'a> Deref for SpannedIter<'a> {
    type Target = Lexer<'a>;

    fn deref(&self) -> &Self::Target {
        &self.lexer
    }
}

impl<'a> DerefMut for SpannedIter<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lexer
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                return Some(Ok(token));
            }

            if self.cursor >= self.input.len() {
                return None;
            }

            let bytes = self.input.as_bytes();
            self.token_start = self.cursor;

            let mut current_state: u16 = 16;
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

                if self.cursor == self.token_start {
                    let next_char = self.input[self.token_start..].chars().next().unwrap();
                    self.cursor = self.token_start + next_char.len_utf8();
                    return Some(Err(()));
                }

                let slice = &self.input[self.token_start..self.cursor];
                let remainder = &self.input[self.cursor..];

                let token = match rule_id {
                0 => continue,
                1 => continue,
                2 => Token::StateKw,
                3 => Token::LetKw,
                4 => Token::RuleKw,
                5 => Token::SkipKw,
                6 => Token::Assign,
                7 => Token::Semi,
                8 => Token::FatArrow,
                9 => Token::Identifier,
                10 => Token::Path,
                11 => {
                    let Ok((tok, extra)) = crate::lexer::rule::handlers::handle_raw_regex(
                        slice, remainder, &mut self.state, &mut self.buffer
                    ) else {
                        return Some(Err(()));
                    };
                    self.cursor += extra;
                    tok
                },
                _ => unreachable!(),
            };

                return Some(Ok(token));
            } else {
                let next_char = self.input[self.token_start..].chars().next().unwrap();
                self.cursor = self.token_start + next_char.len_utf8();
                return Some(Err(()));
            }
        }
    }
}

}

#[allow(unused_imports)]
pub use generated::{Lexer, SpannedIter, Token};
