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

#[allow(unused_imports)]
pub use error::{LexerCompilerDiag, LexerCompilerError};

use std::error::Error;

use codegen::CodeEmitter;
use dfa::{DfaBuilder, DfaOptimizer};
use nfa::NfaBuilder;
use rule::{RuleId, RuleParser};

/// Generates lexer source code from a lexer specification.
/// Returns an error when definition parsing or regex parsing fails.
pub fn generate_lexer_source(input: &str) -> Result<String, Box<dyn Error>> {
    let rule_parser = RuleParser::new(input);
    let (rule, _) = rule_parser.parse();

    let mut nfa_builder = NfaBuilder::new();

    for (id, rule) in rule.rules.iter().enumerate() {
        nfa_builder.add_rule(&rule.ast, RuleId(id));
    }

    let master_nfa = nfa_builder.build();

    let dfa_builder = DfaBuilder::new(&master_nfa);
    let unminimized_dfa = dfa_builder.build();

    let dfa_optimizer = DfaOptimizer::new(unminimized_dfa);
    let optimized_dfa = dfa_optimizer.optimize();

    let emitter = CodeEmitter::new(rule, optimized_dfa);
    let source = emitter.emit();
    Ok(source)
}
