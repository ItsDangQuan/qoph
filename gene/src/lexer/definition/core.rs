use crate::lexer::regex::RegexAst;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PayloadType {
    None,
    StringSlice,
}

#[derive(Debug)]
pub(crate) struct LexerRule<'a> {
    pub(crate) ast: RegexAst<'a>,
    pub(crate) variant_name: String,
    pub(crate) payload: PayloadType,
    pub(crate) priority: usize,
}

#[derive(Debug)]
pub(crate) struct LexerDefinition<'a> {
    pub(crate) rules: Vec<LexerRule<'a>>,
}
