use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let lex_spec = "a*";

    let generated_rust_code = gene::generate_lexer(lex_spec);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lexer.rs");
    fs::write(&dest_path, generated_rust_code).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
