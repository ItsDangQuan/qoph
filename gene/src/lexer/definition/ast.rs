use crate::lexer::regex::RegexAst;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PayloadType {
    None,
    StringSlice,
}

#[derive(Debug)]
pub(super) struct LexerRule {
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

    pub(super) fn ast(&self) -> &RegexAst {
        &self.ast
    }

    pub(super) fn variant_name(&self) -> &str {
        &self.variant_name
    }

    pub(super) fn payload(&self) -> &PayloadType {
        &self.payload
    }

    pub(super) fn priority(&self) -> usize {
        self.priority
    }
}

#[derive(Debug)]
pub(super) struct LexerDefinition {
    rules: Vec<LexerRule>,
}

impl LexerDefinition {
    pub(super) fn new(rules: Vec<LexerRule>) -> Self {
        Self { rules }
    }

    pub(super) fn rules(&self) -> &[LexerRule] {
        &self.rules
    }
}