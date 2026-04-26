use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use gene::generate_lexer_source;

fn temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), nanos))
}

fn compile_program(program_rs: &Path, output_bin: &Path) {
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg(program_rs)
        .arg("-o")
        .arg(output_bin)
        .output()
        .expect("failed to invoke rustc");

    assert!(
        output.status.success(),
        "rustc failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn generate_lexer_source_returns_error_for_invalid_definition() {
    let err = generate_lexer_source("Invalid line without delimiter")
        .expect_err("invalid definitions should fail");

    assert!(err.to_string().contains("Missing '::=' delimiter"));
}

#[test]
fn generated_source_contains_runtime_sections() {
    let src = generate_lexer_source("A ::= a\nB(str) ::= b+").expect("generation should succeed");

    assert!(src.contains("pub enum Token<'a>"));
    assert!(src.contains("pub struct Lexer<'a>"));
    assert!(src.contains("const CLASS_MAP: [u8; 256]"));
    assert!(src.contains("const TRANSITIONS:"));
    assert!(src.contains("const ACCEPT:"));
    assert!(src.contains("impl<'a> Iterator for Lexer<'a>"));
}

#[test]
fn generated_lexer_compiles_and_runs() {
    let spec = "A ::= a\nB(str) ::= b+";
    let generated = generate_lexer_source(spec).expect("generation should succeed");

    let dir = temp_dir("gene_integration_codegen");
    fs::create_dir_all(&dir).expect("temp dir should be creatable");

    let mut program = generated;
    program.push_str(
        r#"

fn main() {
    let mut lexer = Lexer::new("abbbx");
    while let Some(item) = lexer.next() {
        match item {
            Ok(sp) => match sp.node {
                Token::A => println!("A:{}-{}", sp.span.start, sp.span.end),
                Token::B(s) => println!("B:{}:{}-{}", s, sp.span.start, sp.span.end),
            },
            Err(e) => println!("E:{}", e.byte_offset),
        }
    }
}
"#,
    );

    let program_rs = dir.join("main.rs");
    let output_bin = if cfg!(windows) {
        dir.join("lexer_gen.exe")
    } else {
        dir.join("lexer_gen")
    };

    fs::write(&program_rs, program).expect("program file should be writable");

    compile_program(&program_rs, &output_bin);

    let run = Command::new(&output_bin)
        .output()
        .expect("compiled binary should be runnable");
    assert!(run.status.success(), "binary run failed");

    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(stdout.contains("A:0-1"), "stdout was: {stdout}");
    assert!(stdout.contains("B:bbb:1-4"), "stdout was: {stdout}");
    assert!(stdout.contains("E:4"), "stdout was: {stdout}");

    let _ = fs::remove_dir_all(&dir);
}
