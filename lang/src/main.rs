mod glex {
    include!(concat!(env!("OUT_DIR"), "/lexer.rs"));
}
use glex::Lexer;

fn main() {
    let input = r#"5 + 7; // print to console
let var_name = 5 - 7;
let fn_name = fn(a, b) => a * (b / 7);
fn(9, 10); // print to console
"#;
    let l = Lexer::new(input);
    let tokens: Vec<_> = l.filter_map(|res| res.ok()).collect();
    println!("INPUT:\n{}", input);
    println!("TOKENS:\n{:?}", tokens);
}
