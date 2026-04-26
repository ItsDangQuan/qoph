use std::collections::BTreeMap;

use crate::lexer::nfa::{Nfa, Transition};

struct DfaState {
    transitions: [Option<usize>; 256],
    is_accept: bool,
}

impl DfaState {
    fn new() -> Self {
        Self {
            transitions: [None; 256],
            is_accept: false,
        }
    }
}

pub struct Dfa {
    states: Vec<DfaState>,
    start: usize,
}

fn set_bit(bitset: &mut [u64], index: usize) {
    bitset[index / 64] |= 1 << (index % 64)
}

fn test_bit(bitset: &[u64], index: usize) -> bool {
    (bitset[index / 64] & (1 << (index % 64))) != 0
}

fn compute_epsilon_closure(nfa: &Nfa, initial_states: &[usize], bitset_words: usize) -> Vec<u64> {
    let mut closure = vec![0u64; bitset_words];
    let mut stack = Vec::with_capacity(initial_states.len());
    let mut visited = vec![false; nfa.states().len()];

    for &state in initial_states {
        stack.push(state);
        visited[state] = true;
        set_bit(&mut closure, state);
    }

    while let Some(current) = stack.pop() {
        if let &Transition::Epsilon(target1, target2_opt) = nfa.states()[current].transition() {
            if !visited[target1] {
                visited[target1] = true;
                stack.push(target1);
                set_bit(&mut closure, target1);
            }
            if let Some(target2) = target2_opt
                && !visited[target2]
            {
                visited[target2] = true;
                stack.push(target2);
                set_bit(&mut closure, target2);
            }
        }
    }

    closure
}

pub fn compile_dfa(nfa: &Nfa) -> Dfa {
    let num_nfa_states = nfa.states().len();
    let bitset_words = (num_nfa_states + 63) / 64;

    let mut nfa_accept_mask = vec![0u64; bitset_words];
    for (i, state) in nfa.states().iter().enumerate() {
        if matches!(state.transition(), &Transition::Accept) {
            set_bit(&mut nfa_accept_mask, i);
        }
    }

    let mut dfa_states = Vec::new();
    let mut state_mapping: BTreeMap<Vec<u64>, usize> = BTreeMap::new();
    let mut unmarked: Vec<Vec<u64>> = Vec::new();

    let start_subset = compute_epsilon_closure(nfa, &[nfa.start()], bitset_words);
    let start_dfa_idx = 0;

    state_mapping.insert(start_subset.clone(), start_dfa_idx);
    unmarked.push(start_subset.clone());

    let mut start_dfa_state = DfaState::new();
    start_dfa_state.is_accept = start_subset
        .iter()
        .zip(&nfa_accept_mask)
        .any(|(a, b)| (a & b) != 0);

    dfa_states.push(start_dfa_state);

    while let Some(current_subset) = unmarked.pop() {
        let current_dfa_idx = *state_mapping.get(&current_subset).unwrap();

        for byte in 0u8..=255 {
            let mut move_targets = Vec::new();

            for nfa_idx in 0..num_nfa_states {
                if test_bit(&current_subset, nfa_idx)
                    && let &Transition::MatchRange(start_b, end_b, target) =
                        nfa.states()[nfa_idx].transition()
                    && byte >= start_b
                    && byte <= end_b
                {
                    move_targets.push(target);
                }
            }

            if move_targets.is_empty() {
                continue;
            }

            let next_subset = compute_epsilon_closure(nfa, &move_targets, bitset_words);

            let next_dfa_idx = *state_mapping.entry(next_subset.clone()).or_insert_with(|| {
                let new_idx = dfa_states.len();

                let mut new_dfa_state = DfaState::new();
                new_dfa_state.is_accept = next_subset
                    .iter()
                    .zip(&nfa_accept_mask)
                    .any(|(a, b)| (a & b) != 0);

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
