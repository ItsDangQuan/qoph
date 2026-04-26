//! Gene generates lexer and parser code from their definition.

mod lexer;
mod span;

/// Generates lexer source code from a lexer specification.
///
/// The implementation currently returns a stub, but this is the public entrypoint
/// that will eventually consume the internal lexer-definition parser and codegen.
pub fn generate_lexer(input_spec: &str) -> String {
    "pub fn next_token() {/* Generated Lexer */ }".to_string()
}
