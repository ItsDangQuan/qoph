//! DFA construction for lexer rules.

mod bitset;
mod builder;
mod core;
mod optimizer;

#[cfg(test)]
mod tests {
    use crate::lexer::metadata::TokenMetadata;
    use crate::lexer::nfa::NfaBuilder;
    use crate::lexer::regex::RegexParser;

    use super::builder::DfaBuilder;
    use super::core::{Dfa, DfaState};
    use super::optimizer::DfaOptimizer;

    fn parse_regex(input: &str) -> crate::lexer::regex::RegexAst {
        let mut parser = RegexParser::new(input);
        parser.parse().expect("regex should parse")
    }

    fn build_single_rule_dfa(regex: &str, metadata: TokenMetadata) -> Dfa {
        let ast = parse_regex(regex);
        let mut builder = NfaBuilder::new();
        builder.add_rule(&ast, metadata);
        let nfa = builder.build();

        DfaBuilder::new(&nfa).build()
    }

    fn run_dfa(dfa: &Dfa, input: &[u8]) -> Option<TokenMetadata> {
        let mut state = dfa.start;
        for &byte in input {
            match dfa.states[state].transitions[byte as usize] {
                Some(next) => state = next,
                None => return None,
            }
        }
        dfa.states[state].accept
    }

    #[test]
    fn dfa_builder_accepts_exact_literal() {
        let dfa = build_single_rule_dfa("ab", TokenMetadata::new(0, 9));

        assert_eq!(run_dfa(&dfa, b"ab"), Some(TokenMetadata::new(0, 9)));
        assert_eq!(run_dfa(&dfa, b"a"), None);
        assert_eq!(run_dfa(&dfa, b"abc"), None);
    }

    #[test]
    fn dfa_builder_chooses_lowest_priority_acceptance() {
        let ast = parse_regex("a");
        let mut nfa_builder = NfaBuilder::new();
        nfa_builder.add_rule(&ast, TokenMetadata::new(10, 1));
        nfa_builder.add_rule(&ast, TokenMetadata::new(1, 2));
        let nfa = nfa_builder.build();

        let dfa = DfaBuilder::new(&nfa).build();
        assert_eq!(run_dfa(&dfa, b"a"), Some(TokenMetadata::new(1, 2)));
    }

    #[test]
    fn optimizer_merges_equivalent_non_accepting_states() {
        let mut s0 = DfaState::new();
        s0.transitions[b'a' as usize] = Some(1);
        s0.transitions[b'b' as usize] = Some(2);

        let s1 = DfaState::new();
        let s2 = DfaState::new();

        let dfa = Dfa {
            states: vec![s0, s1, s2],
            start: 0,
        };

        let minimized = DfaOptimizer::new(dfa).optimize();
        assert_eq!(minimized.states.len(), 2);
    }

    #[test]
    fn optimizer_keeps_accepting_states_with_different_ids_separate() {
        let mut s0 = DfaState::new();
        s0.transitions[b'a' as usize] = Some(1);
        s0.transitions[b'b' as usize] = Some(2);

        let mut s1 = DfaState::new();
        s1.accept = Some(TokenMetadata::new(0, 11));

        let mut s2 = DfaState::new();
        s2.accept = Some(TokenMetadata::new(0, 12));

        let dfa = Dfa {
            states: vec![s0, s1, s2],
            start: 0,
        };

        let minimized = DfaOptimizer::new(dfa).optimize();
        assert_eq!(minimized.states.len(), 3);
    }
}
