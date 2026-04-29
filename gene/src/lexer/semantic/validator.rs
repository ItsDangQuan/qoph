use std::collections::HashMap;
use std::ops::Range;

use crate::lexer::regex::RegexAst;
use crate::lexer::rule::{FragmentRule, Rule, TokenRule};
use crate::lexer::{LexerCompilerDiag, LexerCompilerError, LexerCompilerOptions};

/// Semantic validator and AST transformer for the parsed lexer rule.
/// It perform four pass:
/// * Collect symbols in the AST and flag colliding symbol.
/// * Verify existence of all references and build dependency graph of references.
/// * Sort references in topological order and flag circular reference.
/// * Resolves and inline fragments into the rule's regex ASTs.
pub(crate) struct SemanticValidator<'a> {
    /// The resolved map of reusable regex fragments.
    pub(crate) fragments: HashMap<&'a str, FragmentRule<'a>>,
    /// The ordered sequence of token rules.
    pub(crate) rules: Vec<TokenRule<'a>>,
    /// Accumulated semantic diagnostics.
    pub(crate) dependencies: HashMap<&'a str, Vec<&'a str>>,
    pub(crate) diagnostics: Vec<LexerCompilerDiag>,
    pub(crate) options: LexerCompilerOptions,
}

#[derive(PartialEq)]
/// State enum for DFS traversal.
enum VisitState {
    Visiting,
    Visited,
}

impl<'a> SemanticValidator<'a> {
    /// Instantiates the validator and executes symbol collection pass.
    pub(crate) fn new(ast: Rule<'a>, options: LexerCompilerOptions) -> Self {
        let mut validator = Self {
            fragments: HashMap::new(),
            rules: Vec::new(),
            dependencies: HashMap::new(),
            diagnostics: Vec::new(),
            options,
        };
        // Pass 1: Symbol collection
        validator.collect_symbols(ast);
        validator
    }

    /// Executes the full semantic validation pipeline.
    pub(crate) fn validate(mut self) -> Result<Rule<'a>, Vec<LexerCompilerDiag>> {
        // Pass 2: Reference verification & dependency graph construction
        self.verify_and_build_graph();

        if !self.diagnostics.is_empty() {
            return Err(self.diagnostics);
        }

        // Pass 3: Topological sort & cycle detection
        let sorted_fragments = match self.topological_sort() {
            Ok(sorted) => sorted,
            Err(()) => return Err(self.diagnostics),
        };

        // Pass 4: Resolves fragments & inline rules
        let resolved_fragments = match self.inline_fragments(sorted_fragments) {
            Ok(resolved) => resolved,
            Err(()) => return Err(self.diagnostics),
        };

        if self.inline_rules(&resolved_fragments).is_err() {
            return Err(self.diagnostics);
        };

        if self.rules.is_empty() {
            self.diagnostics.push(LexerCompilerDiag::new(
                LexerCompilerError::NoRulesDefined,
                0..0,
            ));
            return Err(self.diagnostics);
        }

        Ok(Rule {
            fragments: resolved_fragments.into_values().collect(),
            rules: self.rules,
        })
    }

    /// Traverses the AST and populate internal maps while enforcing one unified namespace.
    fn collect_symbols(&mut self, ast: Rule<'a>) {
        // For detecting cross-category symbol collisions.
        let mut global_symbols: HashMap<_, Range<usize>> = HashMap::new();

        for fragment in ast.fragments {
            if let Some(prev_span) = global_symbols.get(fragment.name) {
                self.diagnostics.push(LexerCompilerDiag::new(
                    LexerCompilerError::CollidingSymbol {
                        name: fragment.name.to_string(),
                        span: prev_span.clone(),
                    },
                    fragment.name_span.clone(),
                ));
            } else {
                global_symbols.insert(fragment.name, fragment.name_span.clone());
                self.fragments.insert(fragment.name, fragment);
            }
        }

        for rule in ast.rules {
            if let Some(prev_span) = global_symbols.get(rule.name) {
                self.diagnostics.push(LexerCompilerDiag::new(
                    LexerCompilerError::CollidingSymbol {
                        name: rule.name.to_string(),
                        span: prev_span.clone(),
                    },
                    rule.name_span.clone(),
                ));
            } else {
                global_symbols.insert(rule.name, rule.name_span.clone());
                self.rules.push(rule);
            }
        }
    }

    /// Top-down extracts reference in regex AST and builds dependency graph of references.
    fn verify_and_build_graph(&mut self) {
        for (&name, fragment) in &self.fragments {
            let mut refs = Vec::new();
            Self::extract_references(&fragment.ast, &mut refs);

            let mut valid_refs = Vec::new();
            for ref_name in refs {
                if self.fragments.contains_key(ref_name) {
                    valid_refs.push(ref_name);
                } else {
                    self.diagnostics.push(LexerCompilerDiag::new(
                        LexerCompilerError::UndefinedSymbol(ref_name.to_string()),
                        fragment.full_span.clone(),
                    ));
                }
            }
            self.dependencies.insert(name, valid_refs);
        }

        for rule in &self.rules {
            let mut refs = Vec::new();
            Self::extract_references(&rule.ast, &mut refs);

            for ref_name in refs {
                if !self.fragments.contains_key(ref_name) {
                    self.diagnostics.push(LexerCompilerDiag::new(
                        LexerCompilerError::UndefinedSymbol(ref_name.to_string()),
                        rule.full_span.clone(),
                    ));
                }
            }
        }
    }

    fn extract_references(ast: &RegexAst<'a>, refs: &mut Vec<&'a str>) {
        match ast {
            RegexAst::FragmentRef(name) => refs.push(name),
            RegexAst::Concatenation(left, right) | RegexAst::Alternation(left, right) => {
                Self::extract_references(left, refs);
                Self::extract_references(right, refs);
            }
            RegexAst::KleeneClosure(inner)
            | RegexAst::PositiveClosure(inner)
            | RegexAst::OptionalClosure(inner) => {
                Self::extract_references(inner, refs);
            }
            RegexAst::Literal(_)
            | RegexAst::Wildcard
            | RegexAst::CharacterClass(_)
            | RegexAst::ComplementClass(_) => {}
        }
    }

    /// Sorts reference topologically for inlining fragments and detect circular reference using DFS.
    fn topological_sort(&mut self) -> Result<Vec<&'a str>, ()> {
        let mut state = HashMap::new();
        let mut sorted = Vec::new();
        let mut cycle_detected = false;

        let nodes: Vec<_> = self.fragments.keys().copied().collect();

        for node in nodes {
            if !state.contains_key(node) {
                self.dfs(node, &mut state, &mut sorted, &mut cycle_detected);
            }
        }

        if cycle_detected { Err(()) } else { Ok(sorted) }
    }

    fn dfs(
        &mut self,
        node: &'a str,
        state: &mut HashMap<&'a str, VisitState>,
        sorted: &mut Vec<&'a str>,
        cycle_detected: &mut bool,
    ) {
        if *cycle_detected {
            return;
        }

        match state.get(node) {
            Some(VisitState::Visiting) => {
                *cycle_detected = true;

                let span = self.fragments.get(node).unwrap().name_span.clone();

                self.diagnostics.push(LexerCompilerDiag::new(
                    LexerCompilerError::CyclicDependency(node.to_string()),
                    span,
                ));
                return;
            }
            Some(VisitState::Visited) => return,
            None => {}
        }

        state.insert(node, VisitState::Visiting);

        if let Some(neighbors) = self.dependencies.get(node).cloned() {
            for neighbor in neighbors {
                self.dfs(neighbor, state, sorted, cycle_detected);
            }
        }

        state.insert(node, VisitState::Visited);

        sorted.push(node);
    }

    /// Resolves fragments into their fully expanded form.
    fn inline_fragments(
        &mut self,
        sorted_order: Vec<&'a str>,
    ) -> Result<HashMap<&'a str, FragmentRule<'a>>, ()> {
        let mut resolved = HashMap::new();
        let mut depths = HashMap::new();

        for name in sorted_order {
            let mut fragment = self.fragments.remove(name).unwrap();

            match Self::replace_refs(
                &mut fragment.ast,
                &resolved,
                &depths,
                self.options.max_expansion_depth,
            ) {
                Ok(depth) => {
                    depths.insert(name, depth);
                    resolved.insert(name, fragment);
                }
                Err(()) => {
                    self.diagnostics.push(LexerCompilerDiag::new(
                        LexerCompilerError::ExpansionDepthExceeded {
                            limit: self.options.max_expansion_depth,
                        },
                        fragment.name_span.clone(),
                    ));
                    return Err(());
                }
            }
        }

        Ok(resolved)
    }

    /// Expands references inside the terminal token rules.
    fn inline_rules(
        &mut self,
        resolved_fragments: &HashMap<&'a str, FragmentRule<'a>>,
    ) -> Result<(), ()> {
        let depths = HashMap::new();
        for rule in &mut self.rules {
            if Self::replace_refs(
                &mut rule.ast,
                resolved_fragments,
                &depths,
                self.options.max_expansion_depth,
            )
            .is_err()
            {
                self.diagnostics.push(LexerCompilerDiag::new(
                    LexerCompilerError::ExpansionDepthExceeded {
                        limit: self.options.max_expansion_depth,
                    },
                    rule.name_span.clone(),
                ));
                return Err(());
            }
        }
        Ok(())
    }

    fn replace_refs(
        ast: &mut RegexAst<'a>,
        resolved: &HashMap<&'a str, FragmentRule<'a>>,
        depths: &HashMap<&'a str, usize>,
        max_depth: usize,
    ) -> Result<usize, ()> {
        match ast {
            RegexAst::FragmentRef(name) => {
                let name = *name;
                if let Some(fragment) = resolved.get(name) {
                    *ast = fragment.ast.clone();

                    let ref_depth = depths.get(name).copied().unwrap_or(0);
                    let new_depth = ref_depth + 1;

                    if new_depth > max_depth {
                        return Err(());
                    }
                    return Ok(new_depth);
                }
                Ok(0)
            }
            RegexAst::Concatenation(left, right) | RegexAst::Alternation(left, right) => {
                let left_depth = Self::replace_refs(left, resolved, depths, max_depth)?;
                let right_depth = Self::replace_refs(right, resolved, depths, max_depth)?;
                Ok(left_depth.max(right_depth))
            }
            RegexAst::KleeneClosure(inner)
            | RegexAst::PositiveClosure(inner)
            | RegexAst::OptionalClosure(inner) => {
                Self::replace_refs(inner, resolved, depths, max_depth)
            }
            RegexAst::Literal(_)
            | RegexAst::Wildcard
            | RegexAst::CharacterClass(_)
            | RegexAst::ComplementClass(_) => Ok(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SemanticValidator;
    use crate::lexer::LexerCompilerError;
    use crate::lexer::rule::RuleParser;

    #[test]
    fn reports_undefined_fragment_in_rules() {
        let input = r#"rule Foo = r"{Missing}";"#;
        let (rule, diag) = RuleParser::new(input).parse();
        assert!(diag.is_empty());

        let err = SemanticValidator::new(rule, Default::default())
            .validate()
            .expect_err("expected undefined symbol");

        assert!(
            err.iter()
                .any(|d| matches!(d.error(), LexerCompilerError::UndefinedSymbol(_)))
        );
    }

    #[test]
    fn reports_cyclic_fragment_dependencies() {
        let input = r#"
let A = r"{B}";
let B = r"{A}";
rule Foo = r"{A}";
"#;
        let (rule, diag) = RuleParser::new(input).parse();
        assert!(diag.is_empty());

        let err = SemanticValidator::new(rule, Default::default())
            .validate()
            .expect_err("expected cycle error");

        assert!(
            err.iter()
                .any(|d| matches!(d.error(), LexerCompilerError::CyclicDependency(_)))
        );
    }

    #[test]
    fn reports_error_when_no_token_rules_defined() {
        let input = r#"let Digit = r"[0-9]";"#;
        let (rule, diag) = RuleParser::new(input).parse();
        assert!(diag.is_empty());

        let err = SemanticValidator::new(rule, Default::default())
            .validate()
            .expect_err("expected no-rules error");

        assert!(
            err.iter()
                .any(|d| matches!(d.error(), LexerCompilerError::NoRulesDefined))
        );
    }
}
