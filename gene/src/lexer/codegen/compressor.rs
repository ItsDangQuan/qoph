use crate::lexer::dfa::Dfa;

pub(super) struct EquivalenceClasses {
    pub(super) class_map: [u8; 256],
    pub(super) compressed_transitions: Vec<Vec<u16>>,
    pub(super) num_classes: usize,
}

impl EquivalenceClasses {
    pub(super) fn compute(dfa: &Dfa) -> Self {
        let num_states = dfa.states.len();
        let mut class_map = [0u8; 256];
        let mut unique_columns = Vec::new();

        #[allow(clippy::needless_range_loop)]
        for byte in 0..=255 {
            let mut column = Vec::with_capacity(num_states);
            for state in &dfa.states {
                let target = match state.transitions[byte] {
                    Some(idx) => (idx + 1) as u16,
                    None => 0,
                };
                column.push(target);
            }

            if let Some(pos) = unique_columns.iter().position(|c| c == &column) {
                class_map[byte] = pos as u8;
            } else {
                class_map[byte] = unique_columns.len() as u8;
                unique_columns.push(column);
            }
        }

        let num_classes = unique_columns.len();

        let mut compressed_transitions = vec![vec![0u16; num_classes]; num_states];
        for state_idx in 0..num_states {
            for class_idx in 0..num_classes {
                compressed_transitions[state_idx][class_idx] = unique_columns[class_idx][state_idx];
            }
        }

        Self {
            class_map,
            compressed_transitions,
            num_classes,
        }
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

    use super::EquivalenceClasses;

    fn compute_classes(spec: &str) -> EquivalenceClasses {
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

        EquivalenceClasses::compute(&dfa)
    }

    #[test]
    fn transitions_to_real_states_are_shifted_above_zero() {
        let classes = compute_classes("A ::= a");

        let class_for_a = classes.class_map[b'a' as usize] as usize;
        let target = classes.compressed_transitions[0][class_for_a];
        assert!(target > 0);
    }

    #[test]
    fn identical_dead_columns_share_the_same_equivalence_class() {
        let classes = compute_classes("A ::= a");

        assert_eq!(
            classes.class_map[b'b' as usize],
            classes.class_map[b'c' as usize]
        );
    }
}
