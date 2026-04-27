//! Lexer-definition parsing and generation internals.
//!
//! Pipeline overview:
//! 1. Parse the textual lexer definition into rule ASTs.
//! 2. Build one NFA branch per rule and merge them into a master NFA.
//! 3. Convert the NFA to a DFA via subset construction.
//! 4. Minimize the DFA.
//! 5. Emit standalone Rust source for a table-driven lexer.

use std::error::Error;

use codegen::RustEmitter;
use definition::DefinitionParser;
use dfa::{DfaBuilder, DfaOptimizer};
use metadata::TokenMetadata;
use nfa::NfaBuilder;

mod codegen;
mod definition;
mod dfa;
mod metadata;
mod nfa;
mod regex;

/// Generates lexer source code from a lexer specification.
///
/// Each non-empty, non-comment line in `input` is parsed as:
///
/// `TokenName[(str)] ::= <regex>`
///
/// where `TokenName` is the emitted enum variant and `(str)` enables
/// a `&str` payload for that token variant.
///
/// Returns an error when definition parsing or regex parsing fails.
pub fn generate_lexer_source(input: &str) -> Result<String, Box<dyn Error>> {
    let definition_parser = DefinitionParser::new(input);
    let definition = definition_parser.parse()?;

    let mut nfa_builder = NfaBuilder::new();

    for (id, rule) in definition.rules.iter().enumerate() {
        let metadata = TokenMetadata {
            priority: rule.priority,
            id,
        };
        nfa_builder.add_rule(&rule.ast, metadata);
    }

    let master_nfa = nfa_builder.build();

    let dfa_builder = DfaBuilder::new(&master_nfa);
    let unminimized_dfa = dfa_builder.build();

    let dfa_optimizer = DfaOptimizer::new(unminimized_dfa);
    let optimized_dfa = dfa_optimizer.optimize();

    let emitter = RustEmitter::new(definition, optimized_dfa);
    let source = emitter.emit()?;
    Ok(source)
}
