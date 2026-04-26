use crate::lexer::dfa::Dfa;

pub(super) struct EquivalenceClasses {
    pub(super) class_map: [u8; 256],
    pub(super) compressed_transitions: Vec<Vec<u16>>,
    pub(super) num_classes: usize,
}

impl EquivalenceClasses {
    pub(super) fn compute(dfa: &Dfa) -> Self {
        let num_states = dfa.states().len();
        let mut class_map = [0u8; 256];
        let mut unique_columns = Vec::new();

        #[allow(clippy::needless_range_loop)]
        for byte in 0..=255 {
            let mut column = Vec::with_capacity(num_states);
            for state in dfa.states() {
                let target = match state.transitions()[byte] {
                    Some(idx) => idx as u16,
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
