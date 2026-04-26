use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lex_spec = r#"
        // 1. Ignored/Trivia Tokens
        Whitespace ::= [ \t\n\r]+
        Comment ::= //[^\n]*

        // 2. Keywords (Must precede Identifier)
        LetKw ::= let
        FnKw ::= fn

        // 3. Multi-Character Operators (Must precede '=' and '>')
        Arrow ::= =>

        // 4. Single-Character Operators & Punctuation
        // Note: Regex metacharacters (+, *, (, )) are escaped.
        Assign ::= =
        Plus ::= \+
        Minus ::= -
        Star ::= \*
        Slash ::= /
        LParen ::= \(
        RParen ::= \)
        Comma ::= ,
        Semi ::= ;

        // 5. Dynamic Payloads (Requires string slice extraction)
        Identifier(str) ::= [a-zA-Z_][a-zA-Z0-9_]*
        Number(str) ::= [0-9]+(\.[0-9]+)?
    "#;

    let generated_rust_code = gene::generate_lexer_source(lex_spec)?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lexer.rs");
    fs::write(&dest_path, generated_rust_code).unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
