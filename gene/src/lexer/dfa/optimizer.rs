use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::core::{Dfa, DfaState};

pub(super) struct DfaOptimizer {
    dfa: Dfa,
    num_states: usize,
}

impl DfaOptimizer {
    pub(super) fn new(dfa: Dfa) -> Self {
        let num_states = dfa.states.len();
        Self { dfa, num_states }
    }

    pub(super) fn optimize(self) -> Dfa {
        if self.num_states == 0 {
            return self.dfa;
        }

        let reverse_transitions = self.build_reverse_transitions();
        let partitions = self.execute_hopcroft(&reverse_transitions);

        self.reconstruct_graph(partitions)
    }

    fn build_reverse_transitions(&self) -> Vec<Vec<Vec<usize>>> {
        let mut reverse = vec![vec![Vec::new(); 256]; self.num_states];
        for src in 0..self.num_states {
            #[allow(clippy::needless_range_loop)]
            for byte in 0..=255 {
                if let Some(target) = self.dfa.states[src].transitions[byte] {
                    reverse[target][byte].push(src);
                }
            }
        }
        reverse
    }

    fn execute_hopcroft(&self, reverse_transitions: &[Vec<Vec<usize>>]) -> Vec<BTreeSet<usize>> {
        let mut partitions = Vec::new();
        let mut state_to_partition = vec![0; self.num_states];

        let mut non_accepting = BTreeSet::new();
        let mut accepting_groups: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();

        for (i, state) in self.dfa.states.iter().enumerate() {
            if let Some(metadata) = &state.accept {
                accepting_groups.entry(metadata.id()).or_default().insert(i);
            } else {
                non_accepting.insert(i);
            }
        }

        if !non_accepting.is_empty() {
            partitions.push(non_accepting);
        }
        for (_, group) in accepting_groups {
            partitions.push(group);
        }

        for (p_idx, partition) in partitions.iter().enumerate() {
            for &state in partition {
                state_to_partition[state] = p_idx;
            }
        }

        let mut worklist: VecDeque<_> = (0..partitions.len()).collect();
        let mut in_worklist = vec![true; partitions.len()];

        while let Some(splitter_idx) = worklist.pop_front() {
            in_worklist[splitter_idx] = false;

            for byte in 0..=255 {
                let mut incoming_states: BTreeSet<usize> = BTreeSet::new();
                for &state in &partitions[splitter_idx] {
                    incoming_states.extend(&reverse_transitions[state][byte]);
                }

                if incoming_states.is_empty() {
                    continue;
                }

                let mut affected_partitions = BTreeSet::new();
                for &state in &incoming_states {
                    affected_partitions.insert(state_to_partition[state]);
                }

                for p_idx in affected_partitions {
                    let partition = &partitions[p_idx];

                    let p1: BTreeSet<_> =
                        partition.intersection(&incoming_states).copied().collect();
                    let p2: BTreeSet<_> = partition.difference(&incoming_states).copied().collect();

                    if p1.is_empty() || p2.is_empty() {
                        continue;
                    }

                    let new_p_idx = partitions.len();
                    partitions[p_idx] = p2.clone();
                    partitions.push(p1.clone());
                    in_worklist.push(false);

                    for &state in &p1 {
                        state_to_partition[state] = new_p_idx;
                    }

                    if in_worklist[p_idx] {
                        worklist.push_back(new_p_idx);
                        in_worklist[new_p_idx] = true;
                    } else if p1.len() <= p2.len() {
                        worklist.push_back(new_p_idx);
                        in_worklist[new_p_idx] = true;
                    } else {
                        worklist.push_back(p_idx);
                        in_worklist[p_idx] = true;
                    }
                }
            }
        }

        partitions
    }

    fn reconstruct_graph(&self, partitions: Vec<BTreeSet<usize>>) -> Dfa {
        let mut state_to_partition = vec![0; self.num_states];
        for (new_idx, partition) in partitions.iter().enumerate() {
            for &old_idx in partition {
                state_to_partition[old_idx] = new_idx;
            }
        }

        let minimized_start = state_to_partition[self.dfa.start];
        let mut minimized_states = Vec::with_capacity(partitions.len());

        for partition in &partitions {
            let rep_idx = *partition.iter().next().unwrap();
            let rep_state = &self.dfa.states[rep_idx];

            let mut new_state = DfaState::new();

            new_state.accept = rep_state.accept;

            for byte in 0..=255 {
                if let Some(target_idx) = rep_state.transitions[byte] {
                    new_state.transitions[byte] = Some(state_to_partition[target_idx]);
                }
            }

            minimized_states.push(new_state);
        }

        Dfa {
            states: minimized_states,
            start: minimized_start,
        }
    }
}