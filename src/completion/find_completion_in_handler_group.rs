use crate::ast::handler::{HandlerDeclaration, HandlerGroupDeclaration};
use crate::ast::reference_space::ReferenceSpace;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::completion::completion_item::CompletionItem;
use crate::completion::find_completion_in_decorator::{find_completion_in_decorator, find_completion_in_empty_decorator};
use crate::completion::find_completion_in_type_expr::{find_completion_in_type_expr, TypeExprFilter};
use crate::traits::has_availability::HasAvailability;
use crate::traits::info_provider::InfoProvider;
use crate::traits::node_trait::NodeTrait;

pub(super) fn find_completion_in_handler_group_declaration(schema: &Schema, source: &Source, handler_group_declaration: &HandlerGroupDeclaration, line_col: (usize, usize)) -> Vec<CompletionItem> {
    for handler_declaration in handler_group_declaration.handler_declarations() {
        if handler_declaration.span.contains_line_col(line_col) {
            return find_completion_in_handler_declaration(schema, source, handler_declaration, line_col);
        }
    }
    vec![]
}

pub(super) fn find_completion_in_handler_declaration(schema: &Schema, source: &Source, handler_declaration: &HandlerDeclaration, line_col: (usize, usize)) -> Vec<CompletionItem> {
    if let Some(input_type) = handler_declaration.input_type() {
        if input_type.span().contains_line_col(line_col) {
            return find_completion_in_type_expr(schema, source, input_type, line_col, &handler_declaration.namespace_str_path(), &vec![], TypeExprFilter::ActionInput, handler_declaration.availability());
        }
    }
    if handler_declaration.output_type().span().contains_line_col(line_col) {
        return find_completion_in_type_expr(schema, source, handler_declaration.output_type(), line_col, &handler_declaration.namespace_str_path(), &vec![], TypeExprFilter::ActionInput, handler_declaration.availability());
    }
    for decorator in handler_declaration.decorators() {
        if decorator.span.contains_line_col(line_col) {
            return find_completion_in_decorator(schema, source, decorator, &handler_declaration.namespace_str_path(), line_col, ReferenceSpace::HandlerDecorator, handler_declaration.availability());
        }
    }
    for empty_decorator in handler_declaration.empty_decorators() {
        if empty_decorator.span.contains_line_col(line_col) {
            return find_completion_in_empty_decorator(schema, source, &handler_declaration.namespace_str_path(), ReferenceSpace::HandlerDecorator, handler_declaration.availability());
        }
    }
    vec![]
}