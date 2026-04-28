use crate::lexer::definition::RuleId;
use crate::lexer::regex::RegexAst;

use super::byte;
use super::core::{Nfa, NfaState, Transition};

pub(crate) struct NfaBuilder {
    states: Vec<NfaState>,
    rule_entry_points: Vec<usize>,
}

impl NfaBuilder {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn add_rule(&mut self, ast: &RegexAst, rule_id: RuleId) {
        let (start, _) = self.build_recursive(ast, rule_id);
        self.rule_entry_points.push(start);
    }

    pub(crate) fn build(mut self) -> Nfa {
        if self.rule_entry_points.is_empty() {
            panic!("cannot build a lexer NFA with zero rules");
        }

        let master_start = self.build_epsilon_tree(self.rule_entry_points.clone());

        Nfa {
            states: self.states,
            start: master_start,
        }
    }

    fn build_recursive(&mut self, kind: &RegexAst, rule_id: RuleId) -> (usize, usize) {
        match kind {
            RegexAst::Literal(c) => {
                let mut buf = [0; 4];
                let bytes = c.encode_utf8(&mut buf).as_bytes();

                let accept = self.add_state(Transition::Accept(rule_id));

                let mut current_target = accept;
                let mut start_state = accept;

                for &b in bytes.iter().rev() {
                    start_state = self.add_state(Transition::MatchRange(b, b, current_target));
                    current_target = start_state;
                }

                (start_state, accept)
            }
            RegexAst::Wildcard => {
                let accept = self.add_state(Transition::Accept(rule_id));
                let mut branch_starts = Vec::with_capacity(byte::wildcard_sequences().len());

                for sequence in byte::wildcard_sequences() {
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
            RegexAst::CharacterClass(char_set) => {
                let accept = self.add_state(Transition::Accept(rule_id));
                let mut branch_starts = Vec::new();

                for range in char_set.ranges() {
                    let byte_sequences = byte::utf8_expand(*range.start(), *range.end());

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
            RegexAst::ComplementClass(char_set) => {
                let inverted_set = char_set.complement();
                let positive_ast_kind = RegexAst::CharacterClass(inverted_set);

                self.build_recursive(&positive_ast_kind, rule_id)
            }
            RegexAst::Concatenation(left, right) => {
                let (left_start, left_accept) = self.build_recursive(left, rule_id);
                let (right_start, right_accept) = self.build_recursive(right, rule_id);

                self.states[left_accept].transition = Transition::Epsilon(right_start, None);

                (left_start, right_accept)
            }
            RegexAst::Alternation(left, right) => {
                let (left_start, left_accept) = self.build_recursive(left, rule_id);
                let (right_start, right_accept) = self.build_recursive(right, rule_id);

                let new_accept = self.add_state(Transition::Accept(rule_id));
                let new_start = self.add_state(Transition::Epsilon(left_start, Some(right_start)));

                self.states[left_accept].transition = Transition::Epsilon(new_accept, None);
                self.states[right_accept].transition = Transition::Epsilon(new_accept, None);

                (new_start, new_accept)
            }
            RegexAst::KleeneClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base, rule_id);

                let new_accept = self.add_state(Transition::Accept(rule_id));
                let new_start = self.add_state(Transition::Epsilon(base_start, Some(new_accept)));

                self.states[base_accept].transition =
                    Transition::Epsilon(base_start, Some(new_accept));

                (new_start, new_accept)
            }
            RegexAst::PositiveClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base, rule_id);

                let new_accept = self.add_state(Transition::Accept(rule_id));

                self.states[base_accept].transition =
                    Transition::Epsilon(base_start, Some(new_accept));

                (base_start, new_accept)
            }
            RegexAst::OptionalClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base, rule_id);

                let new_accept = self.add_state(Transition::Accept(rule_id));
                let new_start = self.add_state(Transition::Epsilon(base_start, Some(new_accept)));

                self.states[base_accept].transition = Transition::Epsilon(new_accept, None);

                (new_start, new_accept)
            }
            RegexAst::FragmentRef(_) => {
                unreachable!("encounter fragment reference despite semantic analyzed")
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

#[cfg(test)]
mod tests {
    use crate::lexer::definition::RuleId;
    use crate::lexer::regex::RegexParser;

    use super::{NfaBuilder, Transition};

    fn parse_regex(input: &str) -> crate::lexer::regex::RegexAst {
        let parser = RegexParser::new(input, 0);
        parser.parse().expect("regex should parse")
    }

    #[test]
    fn build_panics_when_no_rules_are_added() {
        let result = std::panic::catch_unwind(|| NfaBuilder::new().build());
        assert!(result.is_err());
    }

    #[test]
    fn literal_rule_emits_match_then_accept_with_rule_id() {
        let ast = parse_regex("a");
        let rule_id = RuleId(0);

        let mut builder = NfaBuilder::new();
        builder.add_rule(&ast, rule_id);
        let nfa = builder.build();

        let start = nfa.start;
        match nfa.states[start].transition {
            Transition::MatchRange(s, e, next) => {
                assert_eq!((s, e), (b'a', b'a'));
                match nfa.states[next].transition {
                    Transition::Accept(m) => assert_eq!(m, rule_id),
                    _ => panic!("expected accept state"),
                }
            }
            _ => panic!("expected start match-range state"),
        }
    }

    #[test]
    fn empty_character_class_becomes_dead_end_start() {
        let ast = parse_regex("[]");

        let mut builder = NfaBuilder::new();
        builder.add_rule(&ast, RuleId(0));
        let nfa = builder.build();

        assert!(matches!(
            nfa.states[nfa.start].transition,
            Transition::DeadEnd
        ));
    }

    #[test]
    fn multiple_rules_create_master_epsilon_start() {
        let a = parse_regex("a");
        let b = parse_regex("b");

        let mut builder = NfaBuilder::new();
        builder.add_rule(&a, RuleId(0));
        builder.add_rule(&b, RuleId(1));

        let nfa = builder.build();
        assert!(matches!(
            nfa.states[nfa.start].transition,
            Transition::Epsilon(_, Some(_))
        ));
    }
}
