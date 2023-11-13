use crate::ast::middleware::MiddlewareDeclaration;
use crate::{parse_container_node_variables, parse_set_identifier_and_string_path, parse_set_optional};
use crate::parser::parse_argument_list_declaration::parse_argument_list_declaration;
use crate::parser::parse_identifier::parse_identifier;
use crate::parser::parse_span::parse_span;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};

pub(super) fn parse_middleware_declaration(pair: Pair<'_>, context: &mut ParserContext) -> MiddlewareDeclaration {
    let (
        span,
        path,
        mut string_path,
        mut children,
        define_availability,
        actual_availability
    ) = parse_container_node_variables!(pair, context, named, availability);
    let mut identifier = 0;
    let mut argument_list_declaration = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::identifier => parse_set_identifier_and_string_path!(context, current, children, identifier, string_path),
            Rule::argument_list_declaration => parse_set_optional!(parse_argument_list_declaration(current, context), children, argument_list_declaration),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    MiddlewareDeclaration {
        span,
        path,
        string_path,
        children,
        define_availability,
        actual_availability,
        identifier,
        argument_list_declaration,
    }
}
