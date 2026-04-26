mod glex {
    include!(concat!(env!("OUT_DIR"), "/lexer.rs"));
}
use glex::{Lexer, Token};

fn main() {
    let input = r#"5 + 7; // print to console
let var_name = 5 - 7;
let fn_name = fn(a, b) => a * (b / 7);
fn(9, 10); // print to console
"#;
    let l = Lexer::new(input);
    let tokens: Vec<_> = l
        .filter_map(|res| {
            let spanned = res.expect("input is valid");

            match spanned.node {
                Token::Whitespace | Token::Comment => None,
                token => Some(token),
            }
        })
        .collect();
    println!("INPUT:\n{}", input);
    println!("TOKENS:\n{:?}", tokens);
}
