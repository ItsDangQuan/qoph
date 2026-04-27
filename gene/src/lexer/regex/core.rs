use super::char_set::CharSet;

#[derive(Debug, Clone, PartialEq)]
/// The AST node kinds produced by the regex parser.
pub(crate) enum RegexAst {
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
