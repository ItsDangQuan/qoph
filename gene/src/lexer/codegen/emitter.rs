use crate::lexer::{
    codegen::{compressor::EquivalenceClasses, template},
    dfa::Dfa,
    rule::Rule,
};

pub(crate) struct CodeEmitter<'a> {
    rule: Rule<'a>,
    dfa: Dfa,
}

impl<'a> CodeEmitter<'a> {
    pub(crate) fn new(rule: Rule<'a>, dfa: Dfa) -> Self {
        Self { rule, dfa }
    }

    pub(crate) fn emit(&self) -> String {
        let classes = EquivalenceClasses::compute(&self.dfa);

        let tokens_str = self.emit_tokens();
        let class_map_str = self.emit_class_map(&classes);
        let transitions_str = self.emit_transitions(&classes);
        let accept_str = self.emit_accept();
        let match_arms_str = self.emit_match_arms();
        let constructor_str = self.emit_constructor();

        let num_states = (self.dfa.states.len() + 1).to_string();
        let num_classes = classes.num_classes.to_string();

        let start_state = (self.dfa.start + 1).to_string();

        let state_type_str = self.rule.state_type.unwrap_or("()");

        let base_code = template::LEXER_BASE
            .trim_start()
            .replace("{TOKENS}", &tokens_str)
            .replace("{CLASS_MAP}", &class_map_str)
            .replace("{NUM_CLASSES}", &num_classes)
            .replace("{NUM_STATES}", &num_states)
            .replace("{TRANSITIONS}", &transitions_str)
            .replace("{ACCEPT}", &accept_str)
            .replace("{STATE_TYPE}", state_type_str)
            .replace("{CONSTRUCTOR}", &constructor_str);

        let iterator_code = template::ITERATOR_IMPL
            .trim_start()
            .replace("{START_STATE}", &start_state)
            .replace("{MATCH_ARMS}", &match_arms_str);

        let payload = format!("{}\n{}", base_code, iterator_code);

        format!(
            "{}\n{}\n{}",
            template::PREAMBLE.trim_start(),
            payload,
            template::POSTAMBLE.trim_start()
        )
    }

    fn emit_tokens(&self) -> String {
        let mut out = String::new();
        for rule in &self.rule.rules {
            if !rule.skip {
                out.push_str(&format!("    {},\n", rule.name));
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
        for (id, rule) in self.rule.rules.iter().enumerate() {
            if rule.skip {
                out.push_str(&format!(
                    "                {} => {{ return self.next(); }},\n",
                    id
                ));
            } else if let Some(callback_path) = rule.callback {
                let callback = template::CALLBACK.replace("{CALLBACK_PATH}", callback_path);
                out.push_str(&format!("                {} => {{", id));
                out.push_str(&callback);
                out.push_str("                },\n");
            } else {
                out.push_str(&format!(
                    "                {} => Token::{},\n",
                    id, rule.name
                ));
            }
        }
        out.trim_end().to_string()
    }

    fn emit_constructor(&self) -> String {
        let out = if let Some(state_type) = self.rule.state_type {
            template::WITH_STATE_DECL.replace("{STATE_TYPE}", state_type)
        } else {
            template::NEW_DECL.to_string()
        };
        out.trim_start_matches("\n").trim_end().to_string()
    }
}
