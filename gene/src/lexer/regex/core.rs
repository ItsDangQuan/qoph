use std::ops::Range;

use super::charset::CharSet;

#[derive(Debug, Clone, PartialEq)]
/// The AST node kinds produced by the regex parser.
pub(crate) enum RegexAstKind {
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
pub(crate) struct RegexAst {
    pub(crate) kind: RegexAstKind,
    pub(crate) span: Range<usize>,
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::CharSet;

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
}
