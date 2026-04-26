//! UTF-8 expansion helpers for NFA construction.

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

pub(super) fn wildcard_sequences() -> &'static [Utf8Sequence] {
    WILDCARD_SEQUENCES
}

pub(super) fn utf8_expand(start: char, end: char) -> Vec<Vec<(u8, u8)>> {
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

        if s0 + 1 < e0 {
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

#[cfg(test)]
mod tests {
    use super::{utf8_expand, wildcard_sequences};

    #[test]
    fn wildcard_ascii_ranges_exclude_newline() {
        let has_newline = wildcard_sequences()
            .iter()
            .filter(|seq| seq.len() == 1)
            .any(|seq| {
                let (s, e) = seq[0];
                s <= b'\n' && b'\n' <= e
            });

        assert!(!has_newline);
    }

    #[test]
    fn utf8_expand_single_ascii_char_is_single_byte_sequence() {
        let expanded = utf8_expand('A', 'A');
        assert_eq!(expanded, vec![vec![(b'A', b'A')]]);
    }

    #[test]
    fn utf8_expand_across_ascii_boundary_produces_mixed_width_sequences() {
        let expanded = utf8_expand('\u{007F}', '\u{0080}');

        assert!(expanded.iter().any(|seq| seq == &vec![(0x7F, 0x7F)]));
        assert!(expanded
            .iter()
            .any(|seq| seq == &vec![(0xC2, 0xC2), (0x80, 0x80)]));
    }

    #[test]
    fn utf8_expand_skips_surrogate_hole() {
        let expanded = utf8_expand('\u{D7FF}', '\u{E000}');

        assert!(expanded.iter().any(|seq| seq == &vec![(0xED, 0xED), (0x9F, 0x9F), (0xBF, 0xBF)]));
        assert!(expanded.iter().any(|seq| seq == &vec![(0xEE, 0xEE), (0x80, 0x80), (0x80, 0x80)]));
    }
}
