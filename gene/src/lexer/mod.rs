//! Lexer-definition parsing and generation internals.
//!
//! Pipeline overview:
//! 1. Parse the textual lexer definition into rule ASTs.
//! 2. Build one NFA branch per rule and merge them into a master NFA.
//! 3. Convert the NFA to a DFA via subset construction.
//! 4. Minimize the DFA.
//! 5. Emit standalone Rust source for a table-driven lexer.

mod codegen;
mod dfa;
mod error;
mod nfa;
mod regex;
mod rule;
mod semantic;

#[allow(unused_imports)]
pub use error::{LexerCompilerDiag, LexerCompilerError};

use std::error::Error;

use codegen::CodeEmitter;
use dfa::{DfaBuilder, DfaOptimizer};
use nfa::NfaBuilder;
use rule::{RuleId, RuleParser};

use crate::lexer::semantic::SemanticValidator;

#[derive(Debug, Clone)]
pub struct LexerCompilerOptions {
    pub max_expansion_depth: usize,
}

impl Default for LexerCompilerOptions {
    fn default() -> Self {
        Self {
            max_expansion_depth: 16,
        }
    }
}

/// Generates lexer source code from a lexer specification.
/// Returns an error when definition parsing or regex parsing fails.
pub fn generate_lexer_source(
    input: &str,
    options: Option<LexerCompilerOptions>,
) -> Result<String, Vec<LexerCompilerDiag>> {
    let rule_parser = RuleParser::new(input);
    let (rule, diag) = rule_parser.parse();

    if !diag.is_empty() {
        return Err(diag);
    }

    let validator = SemanticValidator::new(rule, options.unwrap_or_default());
    let validated_rule = validator.validate()?;

    let mut nfa_builder = NfaBuilder::new();

    for (id, rule) in validated_rule.rules.iter().enumerate() {
        nfa_builder.add_rule(&rule.ast, RuleId(id));
    }

    let master_nfa = nfa_builder.build();

    let dfa_builder = DfaBuilder::new(&master_nfa);
    let unminimized_dfa = dfa_builder.build();

    let dfa_optimizer = DfaOptimizer::new(unminimized_dfa);
    let optimized_dfa = dfa_optimizer.optimize();

    let emitter = CodeEmitter::new(validated_rule, optimized_dfa);
    let source = emitter.emit();
    Ok(source)
}
