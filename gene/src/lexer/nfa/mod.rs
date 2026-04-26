//! NFA construction for lexer rules.

mod byte;

use crate::lexer::metadata::TokenMetadata;
use crate::lexer::regex::{RegexAst, RegexAstKind};

use self::byte::{utf8_expand, wildcard_sequences};

pub enum Transition {
    Epsilon(usize, Option<usize>),
    MatchRange(u8, u8, usize),
    Accept(TokenMetadata),
    DeadEnd,
}

pub struct NfaState {
    transition: Transition,
}

impl NfaState {
    pub fn transition(&self) -> &Transition {
        &self.transition
    }
}

pub struct Nfa {
    states: Vec<NfaState>,
    start: usize,
}

impl Nfa {
    pub fn states(&self) -> &[NfaState] {
        &self.states
    }

    pub fn start(&self) -> usize {
        self.start
    }
}

pub struct NfaBuilder {
    states: Vec<NfaState>,
    rule_entry_points: Vec<usize>,
}

impl NfaBuilder {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            rule_entry_points: Vec::new(),
        }
    }

    fn add_state(&mut self, transition: Transition) -> usize {
        let index = self.states.len();
        self.states.push(NfaState { transition });
        index
    }

    pub fn add_rule(&mut self, ast: &RegexAst, metadata: TokenMetadata) {
        let (start, _) = self.build_recursive(ast.kind(), metadata);
        self.rule_entry_points.push(start);
    }

    pub fn build(mut self) -> Nfa {
        if self.rule_entry_points.is_empty() {
            panic!("cannot build a lexer NFA with zero rules");
        }

        let master_start = self.build_epsilon_tree(self.rule_entry_points.clone());

        Nfa {
            states: self.states,
            start: master_start,
        }
    }

    fn build_recursive(&mut self, kind: &RegexAstKind, metadata: TokenMetadata) -> (usize, usize) {
        match kind {
            RegexAstKind::Literal(c) => {
                let mut buf = [0; 4];
                let bytes = c.encode_utf8(&mut buf).as_bytes();

                let accept = self.add_state(Transition::Accept(metadata));

                let mut current_target = accept;
                let mut start_state = accept;

                for &b in bytes.iter().rev() {
                    start_state = self.add_state(Transition::MatchRange(b, b, current_target));
                    current_target = start_state;
                }

                (start_state, accept)
            }
            RegexAstKind::Wildcard => {
                let accept = self.add_state(Transition::Accept(metadata));
                let mut branch_starts = Vec::with_capacity(wildcard_sequences().len());

                for sequence in wildcard_sequences() {
                    let mut current_target = accept;
                    let mut seq_start = accept;

                    for &(start_byte, end_byte) in sequence.iter().rev() {
                        seq_start = self.add_state(Transition::MatchRange(
                            start_byte,
                            end_byte,
                            current_target,
                        ));
                        current_target = seq_start;
                    }

                    branch_starts.push(seq_start);
                }

                let final_start = self.build_epsilon_tree(branch_starts);
                (final_start, accept)
            }
            RegexAstKind::CharacterClass(char_set) => {
                let accept = self.add_state(Transition::Accept(metadata));
                let mut branch_starts = Vec::new();

                for range in char_set.ranges() {
                    let byte_sequences = utf8_expand(*range.start(), *range.end());

                    for sequence in byte_sequences {
                        let mut current_target = accept;
                        let mut seq_start = accept;

                        for (start_byte, end_byte) in sequence.into_iter().rev() {
                            seq_start = self.add_state(Transition::MatchRange(
                                start_byte,
                                end_byte,
                                current_target,
                            ));
                            current_target = seq_start;
                        }

                        branch_starts.push(seq_start);
                    }
                }

                let final_start = self.build_epsilon_tree(branch_starts);
                (final_start, accept)
            }
            RegexAstKind::ComplementClass(char_set) => {
                let inverted_set = char_set.complement();
                let positive_ast_kind = RegexAstKind::CharacterClass(inverted_set);

                self.build_recursive(&positive_ast_kind, metadata)
            }
            RegexAstKind::Concatenation(left, right) => {
                let (left_start, left_accept) = self.build_recursive(left.kind(), metadata);
                let (right_start, right_accept) = self.build_recursive(right.kind(), metadata);

                self.states[left_accept].transition = Transition::Epsilon(right_start, None);

                (left_start, right_accept)
            }
            RegexAstKind::Alternation(left, right) => {
                let (left_start, left_accept) = self.build_recursive(left.kind(), metadata);
                let (right_start, right_accept) = self.build_recursive(right.kind(), metadata);

                let new_accept = self.add_state(Transition::Accept(metadata));
                let new_start = self.add_state(Transition::Epsilon(left_start, Some(right_start)));

                self.states[left_accept].transition = Transition::Epsilon(new_accept, None);
                self.states[right_accept].transition = Transition::Epsilon(new_accept, None);

                (new_start, new_accept)
            }
            RegexAstKind::KleeneClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base.kind(), metadata);

                let new_accept = self.add_state(Transition::Accept(metadata));
                let new_start = self.add_state(Transition::Epsilon(base_start, Some(new_accept)));

                self.states[base_accept].transition =
                    Transition::Epsilon(base_start, Some(new_accept));

                (new_start, new_accept)
            }
            RegexAstKind::PositiveClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base.kind(), metadata);

                let new_accept = self.add_state(Transition::Accept(metadata));

                self.states[base_accept].transition =
                    Transition::Epsilon(base_start, Some(new_accept));

                (base_start, new_accept)
            }
            RegexAstKind::OptionalClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base.kind(), metadata);

                let new_accept = self.add_state(Transition::Accept(metadata));
                let new_start = self.add_state(Transition::Epsilon(base_start, Some(new_accept)));

                self.states[base_accept].transition = Transition::Epsilon(new_accept, None);

                (new_start, new_accept)
            }
        }
    }

    fn build_epsilon_tree(&mut self, mut targets: Vec<usize>) -> usize {
        if targets.is_empty() {
            return self.add_state(Transition::DeadEnd);
        }

        if targets.len() == 1 {
            return targets[0];
        }

        while targets.len() > 1 {
            let mut next_level = Vec::new();
            let mut iter = targets.into_iter();

            while let Some(left) = iter.next() {
                if let Some(right) = iter.next() {
                    let parent = self.add_state(Transition::Epsilon(left, Some(right)));
                    next_level.push(parent);
                } else {
                    next_level.push(left);
                }
            }
            targets = next_level;
        }

        targets[0]
    }
}