use super::char_set::CharSet;

#[derive(Debug, Clone, PartialEq)]
/// The AST node kinds produced by the regex parser.
pub(crate) enum RegexAst<'a> {
    /// A literal character.
    Literal(char),
    /// The wildcard dot operator.
    Wildcard,
    /// Concatenation of two expressions.
    Concatenation(Box<RegexAst<'a>>, Box<RegexAst<'a>>),
    /// Alternation between two expressions.
    Alternation(Box<RegexAst<'a>>, Box<RegexAst<'a>>),
    /// Zero-or-more repetition.
    KleeneClosure(Box<RegexAst<'a>>),
    /// One-or-more repetition.
    PositiveClosure(Box<RegexAst<'a>>),
    /// Zero-or-one repetition.
    OptionalClosure(Box<RegexAst<'a>>),
    /// A positive character class.
    CharacterClass(CharSet),
    /// A negated character class.
    ComplementClass(CharSet),
    /// A named fragment defined outside the regex.
    FragmentRef(&'a str),
}
