use std::collections::VecDeque;

use crate::lexer::rule::lexer::Token;

pub(crate) fn handler_raw_regex(
    slice: &str,
    remainder: &str,
    _state: &mut (),
    _buffer: &mut VecDeque<Token>,
) -> Result<(Token, usize), ()> {
    let hash_count = slice.len() - 2;
    let mut search_offset = 0;

    loop {
        let window = &remainder[search_offset..];
        match window.find('"') {
            Some(quote_rel_idx) => {
                let quote_abs_idx = search_offset + quote_rel_idx;
                let hash_start = quote_abs_idx + 1;
                let hash_end = hash_start + hash_count;

                if hash_end <= remainder.len()
                    && remainder[hash_start..hash_end].bytes().all(|b| b == b'#')
                {
                    return Ok((Token::RawRegex, hash_end));
                }
                search_offset = quote_abs_idx + 1;
            }
            None => return Err(()),
        }
    }
}
