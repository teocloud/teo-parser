use crate::ast::config_declaration::ConfigDeclaration;
use crate::{parse_append, parse_container_node_variables, parse_container_node_variables_cleanup, parse_insert, parse_insert_keyword, parse_insert_punctuation, parse_set_optional};
use crate::parser::parse_availability_end::parse_availability_end;
use crate::parser::parse_availability_flag::parse_availability_flag;
use crate::parser::parse_code_comment::parse_code_comment;
use crate::parser::parse_doc_comment::parse_doc_comment;
use crate::parser::parse_field::parse_field;
use crate::parser::parse_identifier::parse_identifier;
use crate::parser::parse_partial_field::parse_partial_field;
use crate::parser::parse_span::parse_span;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};
use crate::traits::identifiable::Identifiable;
use crate::traits::node_trait::NodeTrait;

pub(super) fn parse_config_declaration(pair: Pair<'_>, context: &ParserContext) -> ConfigDeclaration {
    let (
        span,
        path,
        mut string_path,
        mut children,
        define_availability,
        actual_availability
    ) = parse_container_node_variables!(pair, context, named, availability);
    let mut comment: Option<usize> = None;
    let mut identifier: usize = 0;
    let mut fields: Vec<usize> = vec![];
    let mut partial_fields: Vec<usize> = vec![];
    let mut inside_block = false;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::triple_comment_block => if !inside_block {
                parse_set_optional!(parse_doc_comment(current, context), children, comment)
            } else {
                context.insert_unattached_doc_comment(parse_span(&current));
                parse_append!(parse_doc_comment(current, context), children);
            },
            Rule::double_comment_block => parse_append!(parse_code_comment(current, context), children),
            Rule::DECLARE_KEYWORD => parse_insert_keyword!(context, current, children, "declare"),
            Rule::CONFIG_KEYWORD => parse_insert_keyword!(context, current, children, "config"),
            Rule::BLOCK_OPEN => {
                parse_insert_punctuation!(context, current, children, "{");
                inside_block = true;
            },
            Rule::BLOCK_CLOSE => parse_insert_punctuation!(context, current, children, "}"),
            Rule::identifier => {
                let node = parse_identifier(&current, context);
                if context.current_string_path() != vec!["std".to_owned()] {
                    context.insert_error(node.span(), "config declarations are builtin and cannot be declared");
                }
                string_path = context.next_parent_string_path(node.name());
                identifier = node.id();
                children.insert(node.id(), node.into());
            },
            Rule::field_declaration => parse_insert!(parse_field(current, context), children, fields),
            Rule::partial_field => parse_insert!(parse_partial_field(current, context), children, partial_fields),
            Rule::availability_start => parse_append!(parse_availability_flag(current, context), children),
            Rule::availability_end => parse_append!(parse_availability_end(current, context), children),
            Rule::BLOCK_LEVEL_CATCH_ALL => context.insert_unparsed(parse_span(&current)),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    parse_container_node_variables_cleanup!(context, named);
    ConfigDeclaration {
        span,
        path,
        string_path,
        children,
        define_availability,
        actual_availability,
        comment,
        identifier,
        fields,
        partial_fields,
    }
}
