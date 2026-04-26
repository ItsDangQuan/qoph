use crate::lexer::regex::RegexAst;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PayloadType {
    None,
    StringSlice,
}

#[derive(Debug)]
pub(crate) struct LexerRule {
    ast: RegexAst,
    variant_name: String,
    payload: PayloadType,
    priority: usize,
}

impl LexerRule {
    pub(super) fn new(
        ast: RegexAst,
        variant_name: String,
        payload: PayloadType,
        priority: usize,
    ) -> Self {
        Self {
            ast,
            variant_name,
            payload,
            priority,
        }
    }

    pub(crate) fn ast(&self) -> &RegexAst {
        &self.ast
    }

    pub(crate) fn variant_name(&self) -> &str {
        &self.variant_name
    }

    pub(crate) fn payload(&self) -> &PayloadType {
        &self.payload
    }

    pub(crate) fn priority(&self) -> usize {
        self.priority
    }
}

#[derive(Debug)]
pub(crate) struct LexerDefinition {
    rules: Vec<LexerRule>,
}

impl LexerDefinition {
    pub(super) fn new(rules: Vec<LexerRule>) -> Self {
        Self { rules }
    }

    pub(crate) fn rules(&self) -> &[LexerRule] {
        &self.rules
    }
}
