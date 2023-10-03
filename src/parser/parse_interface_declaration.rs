use crate::ast::field::Field;
use crate::ast::interface::InterfaceDeclaration;
use crate::ast::r#type::TypeExpr;
use crate::parser::parse_field::parse_field;
use crate::parser::parse_generics::{parse_generics_constraint, parse_generics_declaration};
use crate::parser::parse_identifier::parse_identifier;
use crate::parser::parse_span::parse_span;
use crate::parser::parse_type_expression::parse_type_expression;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};

pub(super) fn parse_interface_declaration(pair: Pair<'_>, context: &mut ParserContext) -> InterfaceDeclaration {
    let span = parse_span(&pair);
    let mut identifier = None;
    let mut generics_declaration = None;
    let mut generics_constraint = None;
    let mut extends: Vec<TypeExpr> = vec![];
    let mut fields: Vec<Field> = vec![];
    let path = context.next_parent_path();
    let mut string_path = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::identifier => {
                identifier = Some(parse_identifier(&current));
                string_path = Some(context.next_parent_string_path(identifier.as_ref().unwrap().name()));
            },
            Rule::generics_declaration => generics_declaration = Some(parse_generics_declaration(current, context)),
            Rule::type_expression => extends.push(parse_type_expression(current, context)),
            Rule::generics_constraint => generics_constraint = Some(parse_generics_constraint(current, context)),
            Rule::field_declaration => fields.push(parse_field(current, context)),
            _ => (),
        }
    }
    context.pop_parent_id();
    context.pop_string_path();
    InterfaceDeclaration {
        span,
        path,
        string_path: string_path.unwrap(),
        identifier: identifier.unwrap(),
        generics_declaration,
        generics_constraint,
        extends,
        fields,
    }
}
