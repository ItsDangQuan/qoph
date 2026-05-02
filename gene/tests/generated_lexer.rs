use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use gene::generate_lexer_source;

fn temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    env::temp_dir().join(format!("{}_{}", prefix, nanos))
}

fn compile_and_run(spec: &str, top_level: &str, main_body: &str) {
    let generated = generate_lexer_source(spec, None).expect("codegen failed");

    let dir = temp_dir("gene_lexer");
    fs::create_dir_all(&dir).expect("failed to create temp dir");

    let main_path = dir.join("main.rs");
    let main_src = format!(
        "{}\n{}\nfn main() {{\n{}\n}}\n",
        generated, top_level, main_body
    );
    fs::write(&main_path, main_src).expect("failed to write main.rs");

    let mut exe_path = dir.join("lexer_smoke");
    if cfg!(windows) {
        exe_path.set_extension("exe");
    }

    let output = Command::new("rustc")
        .arg(&main_path)
        .arg("--edition=2024")
        .arg("-o")
        .arg(&exe_path)
        .output()
        .expect("failed to invoke rustc");

    assert!(
        output.status.success(),
        "rustc failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = Command::new(&exe_path)
        .output()
        .expect("failed to run generated lexer");

    assert!(
        output.status.success(),
        "lexer run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn generated_lexer_compiles_and_runs() {
    let spec = r#"
        skip rule Ws = r"[ \t\n\r]+";
        rule LetKw = r"let";
        rule Assign = r"=";
        rule Semi = r";";
        rule Number = r"[0-9]+";
        rule Identifier = r"[a-zA-Z_][a-zA-Z0-9_]*";
    "#;

    let main_body = r#"
        use generated::{Lexer, Token};
        let input = "let x=42;";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(item) = lexer.next() {
            match item {
                Ok(tok) => tokens.push(tok),
                Err(()) => std::process::exit(2),
            }
        }

        let expected = vec![
            Token::LetKw,
            Token::Identifier,
            Token::Assign,
            Token::Number,
            Token::Semi,
        ];

        if tokens != expected {
            eprintln!("unexpected tokens: {:?}", tokens);
            std::process::exit(1);
        }
    "#;

    compile_and_run(spec, "", main_body);
}

#[test]
fn generated_code_includes_action_and_excludes_skip_token() {
    let spec = r##"
        skip rule Ws = r"[ \t]+";
        rule Word = r"[a-z]+" => crate::handle_word;
    "##;

    let output = generate_lexer_source(spec, None).expect("codegen failed");
    assert!(output.contains("enum Token"));
    assert!(output.contains("Word"));
    assert!(!output.contains("Ws"));
    assert!(output.contains("crate::handle_word("));
}

#[test]
fn generated_lexer_reports_errors_and_spans() {
    let spec = r#"
        skip rule Ws = r"[ \t\n\r]+";
        rule LetKw = r"let";
        rule Identifier = r"[a-zA-Z_][a-zA-Z0-9_]*";
    "#;

    let main_body = r#"
        use generated::{Lexer, Token};
        let input = "let _bad$";
        let mut lexer = Lexer::new(input).spanned();
        let mut tokens = Vec::new();
        let mut spans = Vec::new();
        let mut errors = 0usize;

        while let Some((item, span)) = lexer.next() {
            match item {
                Ok(tok) => {
                    tokens.push(tok);
                    spans.push(span);
                }
                Err(()) => errors += 1,
            }
        }

        let expected = vec![Token::LetKw, Token::Identifier];
        if tokens != expected {
            eprintln!("unexpected tokens: {:?}", tokens);
            std::process::exit(1);
        }

        if spans != vec![0..3, 4..8] {
            eprintln!("unexpected spans: {:?}", spans);
            std::process::exit(1);
        }

        if errors != 1 {
            eprintln!("unexpected error count: {}", errors);
            std::process::exit(1);
        }
    "#;

    compile_and_run(spec, "", main_body);
}

#[test]
fn generated_lexer_executes_actions() {
    let spec = r##"
        skip rule Ws = r"[ \t\n\r]+";
        rule Word = r"[a-z]+" => crate::handle_word;
    "##;

    let top_level = r#"
        use std::collections::VecDeque;

        pub fn handle_word(
            _slice: &str,
            _remainder: &str,
            _state: &mut (),
            _buffer: &mut VecDeque<Token>
        ) -> Result<(Token, usize), ()> {
            Ok((Token::Word, 0))
        }
    "#;

    let main_body = r#"
        let input = "hello";
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(item) = lexer.next() {
            match item {
                Ok(tok) => tokens.push(tok),
                Err(()) => std::process::exit(2),
            }
        }

        if tokens != vec![Token::Word] {
            eprintln!("unexpected tokens: {:?}", tokens);
            std::process::exit(1);
        }
    "#;

    compile_and_run(spec, top_level, main_body);
}
