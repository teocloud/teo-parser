use crate::ast::argument::Argument;
use crate::ast::argument_list::ArgumentList;
use crate::availability::Availability;
use crate::ast::identifier::Identifier;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::definition::definition::Definition;
use crate::definition::jump_to_definition_in_expression::jump_to_definition_in_expression;
use crate::traits::node_trait::NodeTrait;
use crate::traits::resolved::Resolve;

pub(super) fn jump_to_definition_in_argument_list<'a>(
    schema: &'a Schema,
    source: &'a Source,
    argument_list: &'a ArgumentList,
    namespace_path: &Vec<&'a str>,
    callable_reference: Option<Vec<usize>>,
    line_col: (usize, usize),
    availability: Availability,
) -> Vec<Definition> {
    for argument in argument_list.arguments() {
        if argument.span.contains_line_col(line_col) {
            return jump_to_definition_in_argument(
                schema,
                source,
                argument,
                namespace_path,
                callable_reference,
                line_col,
                availability,
            )
        }
    }
    vec![]
}

pub(super) fn jump_to_definition_in_argument<'a>(
    schema: &'a Schema,
    source: &'a Source,
    argument: &'a Argument,
    namespace_path: &Vec<&'a str>,
    callable_reference: Option<Vec<usize>>,
    line_col: (usize, usize),
    availability: Availability,
) -> Vec<Definition> {
    if let Some(name) = argument.name() {
        if name.span.contains_line_col(line_col) {
            return jump_to_definition_in_argument_name(
                schema,
                source,
                name,
                namespace_path,
                callable_reference,
                line_col,
            );
        }
    }
    if argument.value().span().contains_line_col(line_col) {
        return jump_to_definition_in_expression(
            schema,
            source,
            argument.value(),
            namespace_path,
            line_col,
            argument.value().resolved().r#type(),
            availability,
        );
    }
    vec![]
}

pub(super) fn jump_to_definition_in_argument_name<'a>(
    _schema: &'a Schema,
    _source: &'a Source,
    _name: &'a Identifier,
    _namespace_path: &Vec<&'a str>,
    _callable_reference: Option<Vec<usize>>,
    _line_col: (usize, usize),
) -> Vec<Definition> {
    vec![]
}