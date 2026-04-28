use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Whitespace,
    Comment,
    LetKw,
    RuleKw,
    SkipKw,
    Assign,
    Semi,
    Identifier(&'a str),
    RawRegex(&'a str),
    CodeBlock(&'a str),
}

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

        self.token_start = self.cursor;
        let bytes = self.input.as_bytes();
        let current_char = self.input[self.cursor..].chars().next().unwrap();

        // 1. Trivia Handling (Automatic skipping)
        if current_char.is_whitespace() {
            while self.cursor < self.input.len() {
                let c = self.input[self.cursor..].chars().next().unwrap();
                if c.is_whitespace() {
                    self.cursor += c.len_utf8();
                } else {
                    break;
                }
            }
            return self.next();
        }

        if current_char == '#' {
            while self.cursor < self.input.len() {
                let c = self.input[self.cursor..].chars().next().unwrap();
                self.cursor += c.len_utf8();
                if c == '\n' {
                    break;
                }
            }
            return self.next();
        }

        // 2. Single-Character Punctuation
        if current_char == '=' {
            self.cursor += 1;
            return Some(Ok(Token::Assign));
        }
        if current_char == ';' {
            self.cursor += 1;
            return Some(Ok(Token::Semi));
        }

        // 3. Context-Sensitive Payloads MUST be evaluated before Identifiers
        let tail = &self.input[self.cursor..];
        if tail.starts_with("r#") || tail.starts_with("c#") {
            let is_code_block = tail.starts_with("c#");
            let mut temp_cursor = self.cursor + 1; // Skip 'r' or 'c'
            let mut hash_count = 0;

            while temp_cursor < self.input.len() && bytes[temp_cursor] == b'#' {
                hash_count += 1;
                temp_cursor += 1;
            }

            if temp_cursor < self.input.len()
                && (bytes[temp_cursor] == b'"' || bytes[temp_cursor] == b'`')
            {
                let quote_char = bytes[temp_cursor];
                temp_cursor += 1;

                let mut search_offset = temp_cursor;
                loop {
                    let window = &self.input[search_offset..];
                    if let Some(idx) = window.find(quote_char as char) {
                        let abs_idx = search_offset + idx;
                        let hash_start = abs_idx + 1;
                        let hash_end = hash_start + hash_count;

                        if hash_end <= self.input.len()
                            && self.input[hash_start..hash_end].bytes().all(|b| b == b'#')
                        {
                            self.cursor = hash_end;
                            let slice = &self.input[self.token_start..self.cursor];
                            return Some(Ok(if is_code_block {
                                Token::CodeBlock(slice)
                            } else {
                                Token::RawRegex(slice)
                            }));
                        }
                        search_offset = abs_idx + 1;
                    } else {
                        self.cursor = self.input.len();
                        return Some(Err(LexerError {
                            byte_offset: self.token_start,
                        }));
                    }
                }
            }
        }

        // 4. Identifiers and Keywords
        if current_char.is_ascii_alphabetic() || current_char == '_' {
            while self.cursor < self.input.len() {
                let c = self.input[self.cursor..].chars().next().unwrap();
                if c.is_ascii_alphanumeric() || c == '_' {
                    self.cursor += c.len_utf8();
                } else {
                    break;
                }
            }

            let slice = &self.input[self.token_start..self.cursor];
            let token = match slice {
                "let" => Token::LetKw,
                "rule" => Token::RuleKw,
                "skip" => Token::SkipKw,
                _ => Token::Identifier(slice),
            };
            return Some(Ok(token));
        }

        // 5. Unrecognized Character Path
        self.cursor += current_char.len_utf8();
        Some(Err(LexerError {
            byte_offset: self.token_start,
        }))
    }
}
