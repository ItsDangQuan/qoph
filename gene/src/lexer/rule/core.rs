use std::collections::HashMap;
use std::ops::Range;

use crate::lexer::regex::RegexAst;

#[derive(Debug, Clone)]
pub(crate) struct Rule<'a> {
    /// Reusable regex macro fragments.
    pub(crate) fragments: HashMap<&'a str, FragmentRule<'a>>,

    /// The ordered sequence of token rules.
    pub(crate) rules: Vec<TokenRule<'a>>,
}

#[derive(Debug, Clone)]
pub(crate) struct FragmentRule<'a> {
    pub(crate) name: &'a str,
    pub(crate) ast: RegexAst<'a>,
    pub(crate) span: Range<usize>,
}

#[derive(Debug, Clone)]
pub(crate) struct TokenRule<'a> {
    pub(crate) name: &'a str,
    pub(crate) ast: RegexAst<'a>,
    /// The optional Rust code block (escape hatch) for context-sensitive evaluation.
    pub(crate) action: Option<&'a str>,
    pub(crate) skip: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Rule ID propagated from the lexer definition into automata.
pub(crate) struct RuleId(pub(crate) usize);
