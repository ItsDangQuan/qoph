use std::fmt::{self, Write};

use crate::lexer::definition::{LexerDefinition, PayloadType};
use crate::lexer::dfa::Dfa;

use super::compressor::EquivalenceClasses;

pub(crate) struct RustEmitter {
    definition: LexerDefinition,
    dfa: Dfa,
}

impl RustEmitter {
    pub(crate) fn new(definition: LexerDefinition, dfa: Dfa) -> Self {
        Self { definition, dfa }
    }

    pub(crate) fn emit(&self) -> Result<String, fmt::Error> {
        let mut out = String::new();

        self.emit_headers(&mut out)?;
        self.emit_token_enum(&mut out)?;
        self.emit_tables(&mut out)?;
        self.emit_lexer_struct(&mut out)?;
        self.emit_iterator(&mut out)?;

        Ok(out)
    }

    fn emit_headers(&self, out: &mut String) -> Result<(), fmt::Error> {
        writeln!(out, "// AUTO-GENERATED LEXER")?;
        writeln!(out)?;
        Ok(())
    }

    fn emit_token_enum(&self, out: &mut String) -> Result<(), fmt::Error> {
        writeln!(out, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
        writeln!(out, "pub enum Token<'a> {{")?;

        for rule in &self.definition.rules {
            match rule.payload {
                PayloadType::None => {
                    writeln!(out, "    {},", rule.variant_name)?;
                }
                PayloadType::StringSlice => {
                    writeln!(out, "    {}(&'a str),", rule.variant_name)?;
                }
            }
        }

        writeln!(out, "}}")?;
        writeln!(out)?;
        Ok(())
    }

    fn emit_tables(&self, out: &mut String) -> Result<(), fmt::Error> {
        let num_states = self.dfa.states.len();
        let table_size = num_states + 1;
        let eq_classes = EquivalenceClasses::compute(&self.dfa);

        writeln!(out, "const CLASS_MAP: [u8; 256] = [")?;
        write!(out, "    ")?;
        for (i, &class_id) in eq_classes.class_map.iter().enumerate() {
            write!(out, "{}, ", class_id)?;
            if (i + 1) % 16 == 0 && i < 255 {
                writeln!(out)?;
                write!(out, "    ")?;
            }
        }
        writeln!(out, "\n];\n")?;

        writeln!(
            out,
            "const TRANSITIONS: [[u16; {}]; {}] = [",
            eq_classes.num_classes, table_size
        )?;

        writeln!(out, "    [0; {}],", eq_classes.num_classes)?;

        for state_transitions in eq_classes.compressed_transitions {
            write!(out, "    [")?;
            for (i, &target) in state_transitions.iter().enumerate() {
                write!(out, "{}", target)?;
                if i < eq_classes.num_classes - 1 {
                    write!(out, ", ")?;
                }
            }
            writeln!(out, "],")?;
        }
        writeln!(out, "];\n")?;

        writeln!(out, "const ACCEPT: [Option<u16>; {}] = [", table_size)?;
        writeln!(out, "    None,")?;
        for state in &self.dfa.states {
            match state.accept {
                Some(metadata) => writeln!(out, "    Some({}),", metadata.id)?,
                None => writeln!(out, "    None,")?,
            }
        }
        writeln!(out, "];\n")?;
        Ok(())
    }

    fn emit_lexer_struct(&self, out: &mut String) -> Result<(), fmt::Error> {
        writeln!(out, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
        writeln!(out, "pub struct LexerError {{ pub byte_offset: usize }}")?;
        writeln!(out)?;
        writeln!(out, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
        writeln!(
            out,
            "pub struct Span {{ pub start: usize, pub end: usize }}"
        )?;
        writeln!(out)?;
        writeln!(out, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
        writeln!(
            out,
            "pub struct Spanned<T> {{ pub node: T, pub span: Span }}"
        )?;
        writeln!(out)?;
        writeln!(out, "pub struct Lexer<'a> {{")?;
        writeln!(out, "    input: &'a str,")?;
        writeln!(out, "    cursor: usize,")?;
        writeln!(out, "}}")?;
        writeln!(out)?;
        writeln!(out, "impl<'a> Lexer<'a> {{")?;
        writeln!(out, "    pub fn new(input: &'a str) -> Self {{")?;
        writeln!(out, "        Self {{ input, cursor: 0 }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        writeln!(out)?;
        Ok(())
    }

    fn emit_iterator(&self, out: &mut String) -> Result<(), std::fmt::Error> {
        writeln!(out, "impl<'a> Iterator for Lexer<'a> {{")?;
        writeln!(
            out,
            "    type Item = Result<Spanned<Token<'a>>, LexerError>;"
        )?;
        writeln!(out)?;
        writeln!(out, "    fn next(&mut self) -> Option<Self::Item> {{")?;
        writeln!(
            out,
            "        if self.cursor >= self.input.len() {{ return None; }}"
        )?;
        writeln!(out)?;
        writeln!(out, "        let bytes = self.input.as_bytes();")?;
        writeln!(out, "        let start_cursor = self.cursor;")?;
        writeln!(
            out,
            "        let mut current_state: u16 = {};",
            self.dfa.start + 1
        )?;
        writeln!(out, "        let mut last_accept_state = None;")?;
        writeln!(out, "        let mut last_accept_cursor = self.cursor;")?;
        writeln!(out)?;
        writeln!(out, "        while self.cursor < bytes.len() {{")?;
        writeln!(out, "            let byte = bytes[self.cursor];")?;
        writeln!(out, "            let class_id = CLASS_MAP[byte as usize];")?;
        writeln!(
            out,
            "            let next_state = TRANSITIONS[current_state as usize][class_id as usize];"
        )?;
        writeln!(out)?;
        writeln!(out, "            if next_state == 0 {{ break; }}")?;
        writeln!(out)?;
        writeln!(out, "            current_state = next_state;")?;
        writeln!(out, "            self.cursor += 1;")?;
        writeln!(out)?;
        writeln!(
            out,
            "            if let Some(rule_id) = ACCEPT[current_state as usize] {{"
        )?;
        writeln!(out, "                last_accept_state = Some(rule_id);")?;
        writeln!(out, "                last_accept_cursor = self.cursor;")?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out)?;
        writeln!(out, "        if let Some(rule_id) = last_accept_state {{")?;
        writeln!(out, "            self.cursor = last_accept_cursor;")?;
        writeln!(
            out,
            "            let slice = &self.input[start_cursor..self.cursor];"
        )?;
        writeln!(out)?;
        writeln!(out, "            let token = match rule_id {{")?;

        for (id, rule) in self.definition.rules.iter().enumerate() {
            match rule.payload {
                PayloadType::None => writeln!(
                    out,
                    "                {} => Token::{},",
                    id, rule.variant_name
                )?,
                PayloadType::StringSlice => writeln!(
                    out,
                    "                {} => Token::{}(slice),",
                    id, rule.variant_name
                )?,
            }
        }

        writeln!(out, "                _ => unreachable!(),")?;
        writeln!(out, "            }};")?;
        writeln!(out)?;
        writeln!(
            out,
            "            Some(Ok(Spanned {{ node: token, span: Span {{ start: start_cursor, end: self.cursor }} }}))"
        )?;
        writeln!(out, "        }} else {{")?;
        writeln!(
            out,
            "            let c = self.input[start_cursor..].chars().next().unwrap();"
        )?;
        writeln!(
            out,
            "            self.cursor = start_cursor + c.len_utf8();"
        )?;
        writeln!(
            out,
            "            Some(Err(LexerError {{ byte_offset: start_cursor }}))"
        )?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{
        definition::DefinitionParser,
        dfa::{DfaBuilder, DfaOptimizer},
        metadata::TokenMetadata,
        nfa::NfaBuilder,
    };

    use super::RustEmitter;

    fn emit_source(spec: &str) -> String {
        let definition = DefinitionParser::new(spec)
            .parse()
            .expect("definition should parse");

        let mut nfa_builder = NfaBuilder::new();
        for (id, rule) in definition.rules.iter().enumerate() {
            nfa_builder.add_rule(
                &rule.ast,
                TokenMetadata {
                    priority: rule.priority,
                    id,
                },
            );
        }

        let nfa = nfa_builder.build();
        let dfa = DfaBuilder::new(&nfa).build();
        let dfa = DfaOptimizer::new(dfa).optimize();

        RustEmitter::new(definition, dfa)
            .emit()
            .expect("emission should succeed")
    }

    #[test]
    fn emit_includes_valid_headers() {
        let source = emit_source("A ::= a");

        assert!(source.contains("// AUTO-GENERATED LEXER"));
        assert!(!source.contains("#![allow(dead_code)]"));
        assert!(!source.contains("#![allow(unused_imports)]"));
    }

    #[test]
    fn emit_reserves_zero_dead_row_and_shifts_start_state() {
        let source = emit_source("A ::= a");

        assert!(source.contains("[0; "));
        assert!(source.contains("let mut current_state: u16 = 1;"));
    }

    #[test]
    fn emit_generates_token_variants_for_payload_and_non_payload_rules() {
        let source = emit_source("A ::= a\nB(str) ::= b+");

        assert!(source.contains("A,"));
        assert!(source.contains("B(&'a str),"));
        assert!(source.contains("Token::A"));
        assert!(source.contains("Token::B(slice)"));
    }
}
