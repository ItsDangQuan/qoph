//! Gene generates lexer source code from a compact lexer definition.
//!
//! The definition format is line-oriented and supports reusable fragments,
//! skip rules, and optional Rust code blocks for context-sensitive tokens.
//!
//! Example:
//! ```text
//! # Trivia
//! skip rule Whitespace = r"[ \t\n\r]+";
//!
//! # Fragments
//! let IdentStart = r"[a-zA-Z_]";
//! let IdentRest = r"[a-zA-Z0-9_]*";
//!
//! # Tokens
//! rule LetKw = r"let";
//! rule Identifier = r"{IdentStart}{IdentRest}";
//! rule Number = r"[0-9]+";
//! rule Assign = r"=";
//! rule Semi = r";";
//! ```
//!
//! Generate Rust source code:
//! ```rust
//! # use gene::generate_lexer_source;
//! let spec = r#"rule Number = r"[0-9]+";"#;
//! let source = generate_lexer_source(spec, None).expect("codegen failed");
//! assert!(source.contains("enum Token"));
//! ```

mod lexer;

pub use lexer::{LexerCompilerOptions, generate_lexer_source};
