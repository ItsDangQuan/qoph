use std::ops::RangeInclusive;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
/// A normalized set of character ranges used by character classes.
pub struct CharSet {
    ranges: Vec<RangeInclusive<char>>,
}

impl CharSet {
    /// Creates a character set and merges overlapping or adjacent ranges.
    pub fn new(mut ranges: Vec<RangeInclusive<char>>) -> Self {
        if ranges.is_empty() {
            return CharSet { ranges: Vec::new() };
        }

        ranges.sort_unstable_by_key(|r| *r.start());
        let mut ranges_iter = ranges.into_iter();

        let mut merged = Vec::new();
        let mut current = match ranges_iter.next() {
            Some(r) => r,
            None => unreachable!("ranges was empty despite earlier check"),
        };

        for next in ranges_iter {
            let current_end_val = *current.end();
            let next_start_val = *next.start();

            if next_start_val as u32 <= (current_end_val as u32).saturating_add(1) {
                let max_end = current_end_val.max(*next.end());
                current = *current.start()..=max_end;
            } else {
                merged.push(current);
                current = next;
            }
        }
        merged.push(current);

        Self { ranges: merged }
    }

    /// Returns the ranges covered by this character set.
    pub fn ranges(&self) -> &[RangeInclusive<char>] {
        &self.ranges
    }

    /// Computes the complement of the character set, excluding newline and UTF-16 surrogates.
    pub fn complement(&self) -> CharSet {
        let mut comp_ranges = Vec::new();
        let mut current_start = 0u32;

        for r in &self.ranges {
            let r_start = *r.start() as u32;
            let r_end = *r.end() as u32;

            if current_start < r_start {
                Self::push_gap(&mut comp_ranges, current_start, r_start - 1);
            }

            if r_end == 0x10FFFF {
                current_start = 0x10FFFF + 1;
                break;
            }
            current_start = r_end + 1;
        }

        if current_start <= 0x10FFFF {
            Self::push_gap(&mut comp_ranges, current_start, 0x10FFFF);
        }

        CharSet::new(comp_ranges)
    }

    /// Helper function to split gaps around newline.
    fn push_gap(ranges: &mut Vec<RangeInclusive<char>>, start: u32, end: u32) {
        if start <= 0x0A && end >= 0x0A {
            if start < 0x0A {
                Self::push_valid(ranges, start, 0x09);
            }
            if end > 0x0A {
                Self::push_valid(ranges, 0x0B, end);
            }
        } else {
            Self::push_valid(ranges, start, end);
        }
    }

    /// Helper function to split gaps around the UTF-16 surrogate hole.
    fn push_valid(ranges: &mut Vec<RangeInclusive<char>>, start: u32, end: u32) {
        if start <= 0xDFFF && end >= 0xD800 {
            if start < 0xD800 {
                ranges.push(char::from_u32(start).unwrap()..=char::from_u32(0xD7FF).unwrap());
            }
            if end > 0xDFFF {
                ranges.push(char::from_u32(0xE000).unwrap()..=char::from_u32(end).unwrap());
            }
        } else {
            ranges.push(char::from_u32(start).unwrap()..=char::from_u32(end).unwrap());
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// The AST node kinds produced by the regex parser.
pub enum RegexAstKind {
    /// A literal character.
    Literal(char),
    /// The wildcard dot operator.
    Wildcard,
    /// Concatenation of two expressions.
    Concatenation(Box<RegexAst>, Box<RegexAst>),
    /// Alternation between two expressions.
    Alternation(Box<RegexAst>, Box<RegexAst>),
    /// Zero-or-more repetition.
    KleeneClosure(Box<RegexAst>),
    /// One-or-more repetition.
    PositiveClosure(Box<RegexAst>),
    /// Zero-or-one repetition.
    OptionalClosure(Box<RegexAst>),
    /// A positive character class.
    CharacterClass(CharSet),
    /// A negated character class.
    ComplementClass(CharSet),
}

#[derive(Debug, Clone, PartialEq)]
/// A regex AST node with source span information.
pub struct RegexAst {
    kind: RegexAstKind,
    span: Span,
}

impl RegexAst {
    /// Creates a new AST node.
    pub fn new(kind: RegexAstKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns the node kind.
    pub fn kind(&self) -> &RegexAstKind {
        &self.kind
    }

    /// Returns the source span for this node.
    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use crate::span::{Location, Span};

    use super::{CharSet, RegexAst, RegexAstKind};

    fn as_pairs(ranges: &[RangeInclusive<char>]) -> Vec<(char, char)> {
        ranges.iter().map(|r| (*r.start(), *r.end())).collect()
    }

    #[test]
    fn charset_new_merges_overlapping_and_adjacent_ranges() {
        let set = CharSet::new(vec!['d'..='f', 'a'..='c', 'g'..='g', 'x'..='z']);
        assert_eq!(as_pairs(set.ranges()), vec![('a', 'g'), ('x', 'z')]);
    }

    #[test]
    fn charset_complement_excludes_newline() {
        let set = CharSet::new(vec!['a'..='a']);
        let complement = set.complement();

        assert!(!complement.ranges().iter().any(|r| r.contains(&'\n')));
        assert!(complement.ranges().iter().any(|r| r.contains(&'b')));
    }

    #[test]
    fn charset_complement_of_full_unicode_is_empty() {
        let full = CharSet::new(vec!['\u{0000}'..='\u{10FFFF}']);
        let complement = full.complement();
        assert!(complement.ranges().is_empty());
    }

    #[test]
    fn regex_ast_accessors_return_original_values() {
        let span = Span::new(Location::new(1, 2), Location::new(1, 3));
        let ast = RegexAst::new(RegexAstKind::Literal('x'), span);

        assert!(matches!(ast.kind(), RegexAstKind::Literal('x')));
        assert_eq!(ast.span(), span);
    }
}
