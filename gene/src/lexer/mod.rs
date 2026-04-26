//! Lexer-definition parsing and generation internals.

use std::error::Error;

use crate::lexer::{
    codegen::RustEmitter,
    definition::DefinitionParser,
    dfa::{DfaBuilder, DfaOptimizer},
    metadata::TokenMetadata,
    nfa::NfaBuilder,
};

mod codegen;
mod definition;
mod dfa;
mod metadata;
mod nfa;
mod regex;

/// Generates lexer source code from a lexer specification.
pub fn generate_lexer_source(input: &str) -> Result<String, Box<dyn Error>> {
    let definition_parser = DefinitionParser::new(input);
    let definition = definition_parser.parse()?;

    let mut nfa_builder = NfaBuilder::new();

    for (id, rule) in definition.rules().iter().enumerate() {
        let metadata = TokenMetadata::new(rule.priority(), id);
        nfa_builder.add_rule(rule.ast(), metadata);
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
