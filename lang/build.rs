use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let lex_spec = r#"
        skip rule Whitespace = r"[ \t\n\r]+";
        skip rule Comment = r"//[^\n]*";

        rule LetKw = r"let";
        rule FnKw = r"fn";

        rule Arrow = r"=>";

        rule Assign = r"=";
        rule Plus = r"\+";
        rule Minus = r"-";
        rule Star = r"\*";
        rule Slash = r"/";
        rule LParen = r"\(";
        rule RParen = r"\)";
        rule Comma = r",";
        rule Semi = r";";

        rule Identifier = r"[a-zA-Z_][a-zA-Z0-9_]*";
        rule Number = r"[0-9]+(\.[0-9]+)?";
    "#;

    let generated_rust_code = gene::generate_lexer_source(lex_spec, None).map_err(|errs| {
        errs.iter()
            .map(|err| err.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lexer.rs");
    fs::write(&dest_path, generated_rust_code).unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
