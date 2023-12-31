use crate::ast::config::Config;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::definition::definition::Definition;
use crate::definition::jump_to_definition_in_expression::jump_to_definition_in_expression;
use crate::r#type::r#type::Type;
use crate::search::search_availability::search_availability;
use crate::traits::has_availability::HasAvailability;
use crate::traits::identifiable::Identifiable;
use crate::traits::node_trait::NodeTrait;
use crate::traits::resolved::Resolve;

pub(super) fn jump_to_definition_in_config(schema: &Schema, source: &Source, config: &Config, line_col: (usize, usize)) -> Vec<Definition> {
    let mut namespace_path: Vec<_> = config.string_path.iter().map(|s| s.as_str()).collect();
    namespace_path.pop();
    let availability = search_availability(schema, source, &namespace_path);
    if config.keyword().span.contains_line_col(line_col) {
        if let Some(config_declaration) = schema.find_config_declaration_by_name(config.keyword().name(), config.availability()) {
            return vec![Definition {
                path: schema.source(config_declaration.source_id()).unwrap().file_path.clone(),
                selection_span: config.keyword().span,
                target_span: config_declaration.span,
                identifier_span: config_declaration.identifier().span,
            }];
        } else {
            return vec![];
        }
    }
    for (k, v) in config.items() {
        if k.span().contains_line_col(line_col) {
            if let Some(config_declaration) = schema.find_config_declaration_by_name(config.keyword().name(), config.availability()) {
                if let Some(field) = config_declaration.fields().find(|field| Some(field.identifier().name()) == k.named_key_without_resolving()) {
                    return vec![Definition {
                        path: schema.source(config_declaration.source_id()).unwrap().file_path.clone(),
                        selection_span: k.span(),
                        target_span: field.span,
                        identifier_span: field.identifier().span,
                    }];
                } else {
                    return vec![];
                }
            } else {
                return vec![];
            }
        } else if v.span().contains_line_col(line_col) {
            let undetermined = Type::Undetermined;
            let expected_type = if let Some(config_declaration) = schema.find_config_declaration_by_name(config.keyword().name(), config.availability()) {
                if let Some(field) = config_declaration.fields().find(|field| Some(field.identifier().name()) == k.named_key_without_resolving()) {
                    field.type_expr().resolved()
                } else {
                    &undetermined
                }
            } else {
                &undetermined
            };
            return jump_to_definition_in_expression(
                schema,
                source,
                v,
                &namespace_path,
                line_col,
                expected_type,
                availability
            );
        }
    }
    vec![]
}