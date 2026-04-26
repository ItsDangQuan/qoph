use crate::lexer::regex::{RegexAst, RegexAstKind};

type ByteRange = (u8, u8);
type Utf8Sequence = &'static [ByteRange];

const WILDCARD_SEQUENCES: &[Utf8Sequence] = &[
    // 1-byte sequences (ASCII), excluding 0x0A (\n)
    &[(0x00, 0x09)],
    &[(0x0B, 0x7F)],
    // 2-byte sequences
    &[(0xC2, 0xDF), (0x80, 0xBF)],
    // 3-byte sequences (Split to prevent overlong encodings and surrogates)
    &[(0xE0, 0xE0), (0xA0, 0xBF), (0x80, 0xBF)], // Exclude overlongs
    &[(0xE1, 0xEC), (0x80, 0xBF), (0x80, 0xBF)], // Normal 3-byte
    &[(0xED, 0xED), (0x80, 0x9F), (0x80, 0xBF)], // Exclude surrogates (D800-DFFF)
    &[(0xEE, 0xEF), (0x80, 0xBF), (0x80, 0xBF)], // Normal 3-byte
    // 4-byte sequences (Split to prevent overlongs and out-of-bounds)
    &[(0xF0, 0xF0), (0x90, 0xBF), (0x80, 0xBF), (0x80, 0xBF)], // Exclude overlongs
    &[(0xF1, 0xF3), (0x80, 0xBF), (0x80, 0xBF), (0x80, 0xBF)], // Normal 4-byte
    &[(0xF4, 0xF4), (0x80, 0x8F), (0x80, 0xBF), (0x80, 0xBF)], // Max Unicode (10FFFF)
];

pub enum Transition {
    Epsilon(usize, Option<usize>),
    MatchRange(u8, u8, usize),
    Accept,
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
    accept: usize,
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
}

impl NfaBuilder {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    fn add_state(&mut self, transition: Transition) -> usize {
        let index = self.states.len();
        self.states.push(NfaState { transition });
        index
    }

    pub fn build(mut self, ast: &RegexAst) -> Nfa {
        let (start, accept) = self.build_recursive(&ast.kind());
        Nfa {
            states: self.states,
            start,
            accept,
        }
    }

    fn build_recursive(&mut self, kind: &RegexAstKind) -> (usize, usize) {
        match kind {
            RegexAstKind::Literal(c) => {
                let mut buf = [0; 4];
                let bytes = c.encode_utf8(&mut buf).as_bytes();

                let accept = self.add_state(Transition::Accept);

                let mut current_target = accept;
                let mut start_state = accept;

                for &b in bytes.iter().rev() {
                    start_state = self.add_state(Transition::MatchRange(b, b, current_target));
                    current_target = start_state;
                }

                (start_state, accept)
            }
            RegexAstKind::Wildcard => {
                let accept = self.add_state(Transition::Accept);
                let mut branch_starts = Vec::with_capacity(WILDCARD_SEQUENCES.len());

                for sequence in WILDCARD_SEQUENCES {
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
                let accept = self.add_state(Transition::Accept);
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

                self.build_recursive(&positive_ast_kind)
            }
            RegexAstKind::Concatenation(left, right) => {
                let (left_start, left_accept) = self.build_recursive(left.kind());
                let (right_start, right_accept) = self.build_recursive(right.kind());

                self.states[left_accept].transition = Transition::Epsilon(right_start, None);

                (left_start, right_accept)
            }
            RegexAstKind::Alternation(left, right) => {
                let (left_start, left_accept) = self.build_recursive(left.kind());
                let (right_start, right_accept) = self.build_recursive(right.kind());

                let new_accept = self.add_state(Transition::Accept);
                let new_start = self.add_state(Transition::Epsilon(left_start, Some(right_start)));

                self.states[left_accept].transition = Transition::Epsilon(new_accept, None);
                self.states[right_accept].transition = Transition::Epsilon(new_accept, None);

                (new_start, new_accept)
            }
            RegexAstKind::KleeneClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base.kind());

                let new_accept = self.add_state(Transition::Accept);
                let new_start = self.add_state(Transition::Epsilon(base_start, Some(new_accept)));

                self.states[base_accept].transition =
                    Transition::Epsilon(base_start, Some(new_accept));

                (new_start, new_accept)
            }
            RegexAstKind::PositiveClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base.kind());

                let new_accept = self.add_state(Transition::Accept);

                self.states[base_accept].transition =
                    Transition::Epsilon(base_start, Some(new_accept));

                (base_start, new_accept)
            }
            RegexAstKind::OptionalClosure(base) => {
                let (base_start, base_accept) = self.build_recursive(base.kind());

                let new_accept = self.add_state(Transition::Accept);
                let new_start = self.add_state(Transition::Epsilon(base_start, Some(new_accept)));

                self.states[base_accept].transition = Transition::Epsilon(new_accept, None);

                (new_start, new_accept)
            }
        }
    }

    fn build_epsilon_tree(&mut self, mut targets: Vec<usize>) -> usize {
        debug_assert!(
            !targets.is_empty(),
            "targets from valid regex should not be empty"
        );

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

fn utf8_expand(start: char, end: char) -> Vec<Vec<(u8, u8)>> {
    let mut sequences = Vec::new();
    expand_scalar_range(start as u32, end as u32, &mut sequences);
    sequences
}

fn expand_scalar_range(s: u32, e: u32, seqs: &mut Vec<Vec<(u8, u8)>>) {
    if s <= 0x7F && e > 0x7F {
        expand_scalar_range(s, 0x7F, seqs);
        expand_scalar_range(0x80, e, seqs);
        return;
    }
    if s <= 0x7FF && e > 0x7FF {
        expand_scalar_range(s, 0x7FF, seqs);
        expand_scalar_range(0x800, e, seqs);
        return;
    }
    if s <= 0xFFFF && e > 0xFFFF {
        expand_scalar_range(s, 0xFFFF, seqs);
        expand_scalar_range(0x10000, e, seqs);
        return;
    }

    if s <= 0xD7FF && e >= 0xD800 {
        expand_scalar_range(s, 0xD7FF, seqs);
        expand_scalar_range(0xE000, e, seqs);
        return;
    }

    let s_char = char::from_u32(s).unwrap();
    let e_char = char::from_u32(e).unwrap();

    let mut s_buf = [0; 4];
    let mut e_buf = [0; 4];
    let s_bytes = s_char.encode_utf8(&mut s_buf).as_bytes();
    let e_bytes = e_char.encode_utf8(&mut e_buf).as_bytes();

    expand_byte_sequences(s_bytes, e_bytes, Vec::new(), seqs);
}

fn expand_byte_sequences(
    start: &[u8],
    end: &[u8],
    mut current_prefix: Vec<(u8, u8)>,
    seqs: &mut Vec<Vec<(u8, u8)>>,
) {
    if start.is_empty() {
        seqs.push(current_prefix);
        return;
    }

    let s0 = start[0];
    let e0 = end[0];

    if s0 == e0 {
        current_prefix.push((s0, e0));
        expand_byte_sequences(&start[1..], &end[1..], current_prefix, seqs);
    } else {
        let mut prefix_start = current_prefix.clone();
        prefix_start.push((s0, s0));
        let max_suffix = vec![0xBF; start.len() - 1];
        expand_byte_sequences(&start[1..], &max_suffix, prefix_start, seqs);

        if s0 < e0 - 1 {
            let mut prefix_mid = current_prefix.clone();
            prefix_mid.push((s0 + 1, e0 - 1));

            for _ in 1..start.len() {
                prefix_mid.push((0x80, 0xBF));
            }
            seqs.push(prefix_mid);
        }

        let mut prefix_end = current_prefix;
        prefix_end.push((e0, e0));
        let min_suffix = vec![0x80; end.len() - 1];
        expand_byte_sequences(&min_suffix, &end[1..], prefix_end, seqs);
    }
}
