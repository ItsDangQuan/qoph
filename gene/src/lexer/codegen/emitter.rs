use crate::lexer::{
    codegen::{compressor::EquivalenceClasses, template},
    definition::LexerDef,
    dfa::Dfa,
};

pub(crate) struct CodeEmitter<'a> {
    definition: LexerDef<'a>,
    dfa: Dfa,
}

impl<'a> CodeEmitter<'a> {
    pub(crate) fn new(definition: LexerDef<'a>, dfa: Dfa) -> Self {
        Self { definition, dfa }
    }

    pub(crate) fn emit(&self) -> String {
        let classes = EquivalenceClasses::compute(&self.dfa);

        let tokens_str = self.emit_tokens();
        let class_map_str = self.emit_class_map(&classes);
        let transitions_str = self.emit_transitions(&classes);
        let accept_str = self.emit_accept();
        let match_arms_str = self.emit_match_arms();

        let num_states = (self.dfa.states.len() + 1).to_string();
        let num_classes = classes.num_classes.to_string();

        let start_state = (self.dfa.start + 1).to_string();

        let base_code = template::LEXER_BASE
            .trim_start()
            .replace("{TOKENS}", &tokens_str)
            .replace("{CLASS_MAP}", &class_map_str)
            .replace("{NUM_CLASSES}", &num_classes)
            .replace("{NUM_STATES}", &num_states)
            .replace("{TRANSITIONS}", &transitions_str)
            .replace("{ACCEPT}", &accept_str);

        let iterator_code = template::ITERATOR_IMPL
            .trim_start()
            .replace("{START_STATE}", &start_state)
            .replace("{MATCH_ARMS}", &match_arms_str);

        format!("{}\n{}", base_code, iterator_code)
    }

    fn emit_tokens(&self) -> String {
        let mut out = String::new();
        for rule in &self.definition.rules {
            if !rule.skip {
                out.push_str(&format!("    {}(&'a str),\n", rule.name));
            }
        }
        out.trim_end().to_string()
    }

    fn emit_class_map(&self, classes: &EquivalenceClasses) -> String {
        let mut out = String::new();
        for chunk in classes.class_map.chunks(16) {
            out.push_str("    ");
            for val in chunk {
                out.push_str(&format!("{}, ", val));
            }
            out.push('\n');
        }
        out.trim_end().to_string()
    }

    fn emit_transitions(&self, classes: &EquivalenceClasses) -> String {
        let mut out = String::new();
        out.push_str(&format!("    [0; {}],\n", classes.num_classes));
        for row in &classes.compressed_transitions {
            out.push_str("    [");
            for val in row {
                out.push_str(&format!("{}, ", val));
            }
            out.push_str("],\n");
        }
        out.trim_end().to_string()
    }

    fn emit_accept(&self) -> String {
        let mut out = String::new();
        out.push_str("    None,\n");
        for state in &self.dfa.states {
            if let Some(rule_id) = &state.accept {
                out.push_str(&format!("    Some({}),\n", rule_id.0));
            } else {
                out.push_str("    None,\n");
            }
        }
        out.trim_end().to_string()
    }

    fn emit_match_arms(&self) -> String {
        let mut out = String::new();
        for (id, rule) in self.definition.rules.iter().enumerate() {
            if rule.skip {
                out.push_str(&format!(
                    "                {} => {{ return self.next(); }},\n",
                    id
                ));
            } else if let Some(action) = rule.action {
                out.push_str(&format!("                {} => {{ {} }},\n", id, action));
            } else {
                out.push_str(&format!(
                    "                {} => Token::{}(slice),\n",
                    id, rule.name
                ));
            }
        }
        out.trim_end().to_string()
    }
}
