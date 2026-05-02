pub(super) const PREAMBLE: &str = r#"
#[rustfmt::skip]
#[allow(dead_code)]
mod generated {
"#;

pub(super) const POSTAMBLE: &str = r#"
}

#[allow(unused_imports)]
pub use generated::{Lexer, SpannedIter, Token};
"#;

pub(super) const NEW_DECL: &str = r#"
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: 0,
            token_start: 0,
            buffer: VecDeque::new(),
            state: (),
        }
    }
"#;

pub(super) const WITH_STATE_DECL: &str = r#"
    pub fn with_state(input: &'a str, state: {STATE_TYPE}) -> Self {
        Self {
            input,
            cursor: 0,
            token_start: 0,
            buffer: VecDeque::new(),
            state: state,
        }
    }
"#;

pub(super) const LEXER_BASE: &str = r#"
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut, Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
{TOKENS}
}

const CLASS_MAP: [u8; 256] = [
{CLASS_MAP}
];

const TRANSITIONS: [[u16; {NUM_CLASSES}]; {NUM_STATES}] = [
{TRANSITIONS}
];

const ACCEPT: [Option<usize>; {NUM_STATES}] = [
{ACCEPT}
];

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    token_start: usize,
    buffer: VecDeque<Token>,
    pub state: {STATE_TYPE},
}

impl<'a> Lexer<'a> {
{CONSTRUCTOR}

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
"#;

pub(super) const CALLBACK: &str = r#"
                    let Ok((tok, extra)) = {CALLBACK_PATH}(
                        slice, remainder, &mut self.state, &mut self.buffer
                    ) else {
                        return Some(Err(()));
                    };
                    self.cursor += extra;
                    tok
"#;

pub(super) const ITERATOR_IMPL: &str = r#"
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.buffer.pop_front() {
            return Some(Ok(token));
        }

        if self.cursor >= self.input.len() {
            return None;
        }

        let bytes = self.input.as_bytes();
        self.token_start = self.cursor;

        let mut current_state: u16 = {START_STATE};
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
            let remainder = &self.input[self.cursor..];

            let token = match rule_id {
{MATCH_ARMS}
                _ => unreachable!(),
            };

            Some(Ok(token))
        } else {
            let next_char = self.input[self.token_start..].chars().next().unwrap();
            self.cursor = self.token_start + next_char.len_utf8();
            Some(Err(()))
        }
    }
}
"#;
