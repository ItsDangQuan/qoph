// AUTO-GENERATED LEXER

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Whitespace,
    Comment,
    LetKw,
    RuleKw,
    Assign,
    Semi,
    Identifier(&'a str),
    RawRegex(&'a str),
}

const CLASS_MAP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 3, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    1, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 6, 0, 7, 0, 0, 
    0, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 8, 
    9, 8, 8, 8, 8, 10, 8, 8, 8, 8, 8, 8, 11, 8, 8, 8, 
    8, 8, 12, 8, 13, 14, 8, 8, 8, 8, 8, 0, 0, 0, 0, 0, 
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 
    17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 
    17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 
    18, 18, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 
    19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 
    20, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 22, 21, 21, 
    23, 24, 24, 24, 25, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 
];

const TRANSITIONS: [[u16; 26]; 40] = [
    [0; 26],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 20, 20, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 11, 11, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [20, 20, 0, 3, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 0, 0, 0, 0, 1, 18, 19, 17, 30, 31, 16],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [10, 10, 0, 10, 10, 10, 10, 10, 10, 12, 10, 10, 10, 10, 10, 0, 0, 0, 0, 27, 35, 34, 33, 39, 38, 37],
    [0, 11, 11, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 8, 0, 28, 0, 0, 28, 0, 22, 28, 28, 28, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 8, 26, 28, 0, 0, 28, 0, 28, 28, 28, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 11, 11, 11, 20, 0, 24, 23, 28, 0, 28, 36, 14, 28, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [20, 20, 0, 3, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 0, 0, 0, 0, 1, 18, 19, 17, 30, 31, 16],
    [0, 0, 0, 4, 0, 28, 0, 0, 28, 0, 28, 28, 28, 28, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 5, 0, 28, 0, 0, 28, 0, 28, 28, 28, 28, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 8, 0, 28, 0, 0, 28, 0, 28, 28, 28, 28, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 8, 0, 28, 0, 0, 28, 0, 28, 13, 28, 28, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 19, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 19, 19, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 8, 0, 28, 0, 0, 28, 0, 28, 28, 28, 21, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 27, 27, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 8, 0, 28, 0, 0, 28, 0, 32, 28, 28, 28, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 34, 34, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 34, 0, 0, 0, 0, 0, 0, 0, 0],
];

const ACCEPT: [Option<u16>; 40] = [
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
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError { pub byte_offset: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span { pub start: usize, pub end: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> { pub node: T, pub span: Span }

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Spanned<Token<'a>>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.input.len() { return None; }

        let bytes = self.input.as_bytes();
        let start_cursor = self.cursor;
        let mut current_state: u16 = 15;
        let mut last_accept_state = None;
        let mut last_accept_cursor = self.cursor;

        while self.cursor < bytes.len() {
            let byte = bytes[self.cursor];
            let class_id = CLASS_MAP[byte as usize];
            let next_state = TRANSITIONS[current_state as usize][class_id as usize];

            if next_state == 0 { break; }

            current_state = next_state;
            self.cursor += 1;

            if let Some(rule_id) = ACCEPT[current_state as usize] {
                last_accept_state = Some(rule_id);
                last_accept_cursor = self.cursor;
            }
        }

        if let Some(rule_id) = last_accept_state {
            self.cursor = last_accept_cursor;
            let slice = &self.input[start_cursor..self.cursor];

            let token = match rule_id {
                0 => Token::Whitespace,
                1 => Token::Comment,
                2 => Token::LetKw,
                3 => Token::RuleKw,
                4 => Token::Assign,
                5 => Token::Semi,
                6 => Token::Identifier(slice),
                7 => Token::RawRegex(slice),
                _ => unreachable!(),
            };

            Some(Ok(Spanned { node: token, span: Span { start: start_cursor, end: self.cursor } }))
        } else {
            let c = self.input[start_cursor..].chars().next().unwrap();
            self.cursor = start_cursor + c.len_utf8();
            Some(Err(LexerError { byte_offset: start_cursor }))
        }
    }
}
