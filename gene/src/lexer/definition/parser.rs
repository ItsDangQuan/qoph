use crate::lexer::regex::RegexParser;

use super::core::{LexerDefinition, LexerRule, PayloadType};
use super::error::{DefinitionParserError, DefinitionParserErrorKind, DefinitionParserResult};

pub(crate) struct DefinitionParser<'a> {
    input: &'a str,
}

impl<'a> DefinitionParser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub(crate) fn parse(&self) -> DefinitionParserResult<LexerDefinition> {
        let mut rules = Vec::new();
        let mut priority = 0;
        let mut line_start = 0;

        for line_with_newline in self.input.split_inclusive('\n') {
            let line = line_with_newline
                .strip_suffix('\n')
                .unwrap_or(line_with_newline);

            if line.trim().is_empty() || line.trim_start().starts_with("//") {
                line_start += line_with_newline.len();
                continue;
            }

            let split_byte_idx = match line.find("::=") {
                Some(idx) => idx,
                None => {
                    let leading_whitespace = line.len() - line.trim_start().len();
                    return Err(DefinitionParserError::new(
                        DefinitionParserErrorKind::MissingDelimiter,
                        line_start + leading_whitespace,
                    ));
                }
            };

            let left_side = line[..split_byte_idx].trim();
            let after_delimiter = &line[split_byte_idx + 3..];
            let regex_str = after_delimiter.trim_start();

            let whitespace_offset = after_delimiter.len() - regex_str.len();
            let regex_byte_start = split_byte_idx + 3 + whitespace_offset;

            let regex_loc = line_start + regex_byte_start;

            if regex_str.is_empty() {
                return Err(DefinitionParserError::new(
                    DefinitionParserErrorKind::EmptyRegex,
                    regex_loc,
                ));
            }

            let category_loc = line_start + (line.len() - line.trim_start().len());

            let (variant_name, payload) = Self::parse_category(left_side, category_loc)?;

            let mut regex_parser = RegexParser::with_location(regex_str, regex_loc);
            let ast = match regex_parser.parse() {
                Ok(ast) => ast,
                Err(e) => {
                    return Err(DefinitionParserError::new(
                        DefinitionParserErrorKind::RegexParseError(e),
                        regex_loc,
                    ));
                }
            };

            rules.push(LexerRule {
                ast,
                variant_name,
                payload,
                priority,
            });

            priority += 1;
            line_start += line_with_newline.len();
        }

        if rules.is_empty() {
            return Err(DefinitionParserError::new(
                DefinitionParserErrorKind::NoRules,
                0,
            ));
        }

        Ok(LexerDefinition { rules })
    }

    fn parse_category(
        input: &str,
        location: usize,
    ) -> DefinitionParserResult<(String, PayloadType)> {
        if input.is_empty() {
            return Err(DefinitionParserError::new(
                DefinitionParserErrorKind::EmptyCategory,
                location,
            ));
        }

        let open_count = input.chars().filter(|&c| c == '(').count();
        let close_count = input.chars().filter(|&c| c == ')').count();

        if open_count == 0 && close_count > 0 {
            return Err(DefinitionParserError::new(
                DefinitionParserErrorKind::MismatchedParentheses,
                location,
            ));
        }

        if open_count > 0 && close_count == 0 {
            return Err(DefinitionParserError::new(
                DefinitionParserErrorKind::UnclosedPayload,
                location,
            ));
        }

        if open_count > 0 || close_count > 0 {
            if open_count != 1 || close_count != 1 {
                return Err(DefinitionParserError::new(
                    DefinitionParserErrorKind::MismatchedParentheses,
                    location,
                ));
            }

            let open_paren = input
                .find('(')
                .expect("open_count == 1 guarantees '(' exists");
            let close_paren = input
                .find(')')
                .expect("close_count == 1 guarantees ')' exists");

            if close_paren < open_paren || close_paren != input.len() - 1 {
                return Err(DefinitionParserError::new(
                    DefinitionParserErrorKind::MismatchedParentheses,
                    location,
                ));
            }

            let variant_name = input[..open_paren].trim().to_string();
            let payload_str = input[open_paren + 1..close_paren].trim();

            if variant_name.is_empty() {
                return Err(DefinitionParserError::new(
                    DefinitionParserErrorKind::EmptyCategory,
                    location,
                ));
            }

            let payload_type = match payload_str {
                "str" => PayloadType::StringSlice,
                _ => {
                    return Err(DefinitionParserError::new(
                        DefinitionParserErrorKind::UnknownPayloadType(payload_str.to_string()),
                        location,
                    ));
                }
            };

            Ok((variant_name, payload_type))
        } else {
            Ok((input.to_string(), PayloadType::None))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::regex::RegexAstKind;

    use super::{
        DefinitionParser, DefinitionParserError, DefinitionParserErrorKind, LexerDefinition,
        PayloadType,
    };

    fn parse_ok(input: &str) -> LexerDefinition {
        DefinitionParser::new(input)
            .parse()
            .expect("definition parse should succeed")
    }

    fn parse_err(input: &str) -> DefinitionParserError {
        DefinitionParser::new(input)
            .parse()
            .expect_err("definition parse should fail")
    }

    #[test]
    fn parses_single_rule_without_payload() {
        let def = parse_ok("Ident ::= a");
        assert_eq!(def.rules.len(), 1);

        let rule = &def.rules[0];
        assert_eq!(rule.variant_name, "Ident");
        assert_eq!(rule.payload, PayloadType::None);
        assert_eq!(rule.priority, 0);
        assert!(matches!(rule.ast.kind, RegexAstKind::Literal('a')));
    }

    #[test]
    fn parses_string_payload_rule() {
        let def = parse_ok("Ident(str) ::= [a-z]+");
        assert_eq!(def.rules.len(), 1);
        assert_eq!(def.rules[0].variant_name, "Ident");
        assert_eq!(def.rules[0].payload, PayloadType::StringSlice);
        assert_eq!(def.rules[0].priority, 0);
    }

    #[test]
    fn parses_multiple_rules_and_assigns_priority_by_order() {
        let input = "\n// comment\n\nA ::= a\n  // ignored\nB(str) ::= b+\nC ::= c\n";
        let def = parse_ok(input);

        assert_eq!(def.rules.len(), 3);
        assert_eq!(def.rules[0].variant_name, "A");
        assert_eq!(def.rules[0].priority, 0);
        assert_eq!(def.rules[1].variant_name, "B");
        assert_eq!(def.rules[1].payload, PayloadType::StringSlice);
        assert_eq!(def.rules[1].priority, 1);
        assert_eq!(def.rules[2].variant_name, "C");
        assert_eq!(def.rules[2].priority, 2);
    }

    #[test]
    fn reports_no_rules_for_empty_or_comment_only_input() {
        let error = parse_err("\n// only comments\n  // and spaces\n");
        assert!(matches!(error.kind(), DefinitionParserErrorKind::NoRules));
        assert_eq!(error.location(), 0);
    }

    #[test]
    fn reports_missing_delimiter_with_leading_whitespace_column() {
        let error = parse_err("   Ident a");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::MissingDelimiter
        ));
        assert_eq!(error.location(), 3);
    }

    #[test]
    fn reports_empty_regex_at_regex_start_column() {
        let error = parse_err("Ident ::=    ");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::EmptyRegex
        ));
        assert_eq!(error.location(), 13);
    }

    #[test]
    fn reports_empty_category() {
        let error = parse_err("   ::= a");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::EmptyCategory
        ));
        assert_eq!(error.location(), 3);
    }

    #[test]
    fn reports_unclosed_payload() {
        let error = parse_err("Ident(str ::= a");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::UnclosedPayload
        ));
    }

    #[test]
    fn reports_mismatched_parentheses_unmatched_close() {
        let error = parse_err("Ident) ::= a");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::MismatchedParentheses
        ));
    }

    #[test]
    fn reports_mismatched_parentheses_trailing_garbage_after_payload() {
        let error = parse_err("Ident(str)extra ::= a");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::MismatchedParentheses
        ));
    }

    #[test]
    fn reports_mismatched_parentheses_multiple_groups() {
        let error = parse_err("Ident(str)(x) ::= a");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::MismatchedParentheses
        ));
    }

    #[test]
    fn reports_unknown_payload_type() {
        let error = parse_err("Ident(i64) ::= a");
        match error.kind() {
            DefinitionParserErrorKind::UnknownPayloadType(t) => assert_eq!(t, "i64"),
            _ => panic!("expected unknown payload type"),
        }
    }

    #[test]
    fn wraps_regex_parser_errors() {
        let error = parse_err("Ident ::= (a");
        assert!(matches!(
            error.kind(),
            DefinitionParserErrorKind::RegexParseError(_)
        ));
    }

    #[test]
    fn display_for_no_rules_is_human_readable() {
        let error = parse_err("\n// none\n");
        assert_eq!(
            error.to_string(),
            "Lexer definition contains no valid rules."
        );
    }

    #[test]
    fn display_includes_line_and_column_for_local_errors() {
        let error = parse_err("Ident(str)extra ::= a");
        let message = error.to_string();
        assert!(message.contains("Error at byte 0:"));
        assert!(message.contains("Mismatched payload parentheses."));
    }
}
