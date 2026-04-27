use std::collections::BTreeMap;

use crate::lexer::metadata::TokenMetadata;
use crate::lexer::nfa::{Nfa, Transition};

use super::bitset::BitSet;
use super::core::{Dfa, DfaState};

pub(crate) struct DfaBuilder<'a> {
    nfa: &'a Nfa,
    num_nfa_states: usize,
    bitset_words: usize,
}

impl<'a> DfaBuilder<'a> {
    pub(crate) fn new(nfa: &'a Nfa) -> Self {
        let num_nfa_states = nfa.states.len();
        Self {
            nfa,
            num_nfa_states,
            bitset_words: num_nfa_states.div_ceil(64),
        }
    }

    pub(crate) fn build(self) -> Dfa {
        let mut dfa_states = Vec::new();
        let mut state_mapping = BTreeMap::new();
        let mut unmarked = Vec::new();

        let start_subset = self.compute_epsilon_closure(&[self.nfa.start]);
        let start_dfa_idx = 0;

        state_mapping.insert(start_subset.clone(), start_dfa_idx);
        unmarked.push(start_subset.clone());

        let mut start_dfa_state = DfaState::new();
        start_dfa_state.accept = self.resolve_acceptance(&start_subset);
        dfa_states.push(start_dfa_state);

        while let Some(current_subset) = unmarked.pop() {
            let current_dfa_idx = *state_mapping.get(&current_subset).unwrap();

            for byte in 0..=255 {
                let mut move_targets = Vec::new();

                for nfa_idx in 0..self.num_nfa_states {
                    if current_subset.test(nfa_idx)
                        && let Transition::MatchRange(start_b, end_b, target) =
                            self.nfa.states[nfa_idx].transition
                        && byte >= start_b
                        && byte <= end_b
                    {
                        move_targets.push(target);
                    }
                }

                if move_targets.is_empty() {
                    continue;
                }

                let next_subset = self.compute_epsilon_closure(&move_targets);

                let next_dfa_idx = *state_mapping.entry(next_subset.clone()).or_insert_with(|| {
                    let new_idx = dfa_states.len();

                    let mut new_dfa_state = DfaState::new();
                    new_dfa_state.accept = self.resolve_acceptance(&next_subset);

                    dfa_states.push(new_dfa_state);
                    unmarked.push(next_subset);

                    new_idx
                });

                dfa_states[current_dfa_idx].transitions[byte as usize] = Some(next_dfa_idx);
            }
        }

        Dfa {
            states: dfa_states,
            start: start_dfa_idx,
        }
    }

    fn compute_epsilon_closure(&self, initial_states: &[usize]) -> BitSet {
        let mut closure = BitSet::new(self.bitset_words);
        let mut stack = Vec::with_capacity(initial_states.len());
        let mut visited = vec![false; self.num_nfa_states];

        for &state in initial_states {
            stack.push(state);
            visited[state] = true;
            closure.set(state);
        }

        while let Some(current) = stack.pop() {
            if let Transition::Epsilon(target1, target2_opt) = self.nfa.states[current].transition {
                if !visited[target1] {
                    visited[target1] = true;
                    stack.push(target1);
                    closure.set(target1);
                }
                if let Some(target2) = target2_opt
                    && !visited[target2]
                {
                    visited[target2] = true;
                    stack.push(target2);
                    closure.set(target2);
                }
            }
        }

        closure
    }

    fn resolve_acceptance(&self, subset: &BitSet) -> Option<TokenMetadata> {
        let mut best_match = None;

        for nfa_idx in 0..self.num_nfa_states {
            if subset.test(nfa_idx)
                && let Transition::Accept(metadata) = self.nfa.states[nfa_idx].transition
            {
                match best_match {
                    None => best_match = Some(metadata),
                    Some(existing) => {
                        if metadata.priority < existing.priority {
                            best_match = Some(metadata);
                        }
                    }
                }
            }
        }

        best_match
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::metadata::TokenMetadata;
    use crate::lexer::nfa::NfaBuilder;
    use crate::lexer::regex::RegexParser;

    use super::super::core::{Dfa, DfaState};
    use super::DfaBuilder;

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
        let dfa = build_single_rule_dfa("ab", TokenMetadata { priority: 0, id: 9 });

        assert_eq!(
            run_dfa(&dfa, b"ab"),
            Some(TokenMetadata { priority: 0, id: 9 })
        );
        assert_eq!(run_dfa(&dfa, b"a"), None);
        assert_eq!(run_dfa(&dfa, b"abc"), None);
    }

    #[test]
    fn dfa_builder_chooses_lowest_priority_acceptance() {
        let ast = parse_regex("a");
        let mut nfa_builder = NfaBuilder::new();
        nfa_builder.add_rule(
            &ast,
            TokenMetadata {
                priority: 10,
                id: 1,
            },
        );
        nfa_builder.add_rule(&ast, TokenMetadata { priority: 1, id: 2 });
        let nfa = nfa_builder.build();

        let dfa = DfaBuilder::new(&nfa).build();
        assert_eq!(
            run_dfa(&dfa, b"a"),
            Some(TokenMetadata { priority: 1, id: 2 })
        );
    }

    #[test]
    fn dfa_builder_handles_multiple_branches() {
        let ast = parse_regex("a|b");
        let mut nfa_builder = NfaBuilder::new();
        nfa_builder.add_rule(&ast, TokenMetadata { priority: 0, id: 1 });
        let nfa = nfa_builder.build();

        let dfa = DfaBuilder::new(&nfa).build();
        assert_eq!(
            run_dfa(&dfa, b"a"),
            Some(TokenMetadata { priority: 0, id: 1 })
        );
        assert_eq!(
            run_dfa(&dfa, b"b"),
            Some(TokenMetadata { priority: 0, id: 1 })
        );
    }

    #[test]
    fn dfa_builder_rejects_unmatched_input() {
        let dfa = build_single_rule_dfa("ab", TokenMetadata { priority: 0, id: 9 });
        assert_eq!(run_dfa(&dfa, b"ac"), None);
    }

    #[test]
    fn core_state_can_be_manually_assembled_for_testing() {
        let mut state = DfaState::new();
        state.transitions[b'a' as usize] = Some(1);
        assert_eq!(state.transitions[b'a' as usize], Some(1));
    }
}
