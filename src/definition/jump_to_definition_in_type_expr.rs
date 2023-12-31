use crate::availability::Availability;
use crate::ast::generics::GenericsDeclaration;
use crate::ast::reference_space::ReferenceSpace;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::ast::type_expr::{TypedShape, TypeExprKind, TypeItem};
use crate::definition::definition::Definition;
use crate::definition::jump_to_definition_in_model::jump_to_definition_in_model;
use crate::search::search_identifier_path::{search_identifier_path_names_with_filter_to_path};
use crate::traits::identifiable::Identifiable;
use crate::traits::node_trait::NodeTrait;
use crate::utils::top_filter::top_filter_for_reference_type;

pub(super) fn jump_to_definition_in_type_expr_kind(
    schema: &Schema,
    source: &Source,
    type_expr: &TypeExprKind,
    namespace_path: &Vec<&str>,
    line_col: (usize, usize),
    generics_declarations: &Vec<&GenericsDeclaration>,
    availability: Availability,
) -> Vec<Definition> {
    match type_expr {
        TypeExprKind::Expr(type_expr) => jump_to_definition_in_type_expr_kind(
            schema,
            source,
            type_expr.as_ref(),
            namespace_path,
            line_col,
            generics_declarations,
            availability
        ),
        TypeExprKind::BinaryOp(b) => if b.lhs().span().contains_line_col(line_col) {
            jump_to_definition_in_type_expr_kind(
                schema,
                source,
                &b.lhs().kind,
                namespace_path,
                line_col,
                generics_declarations,
                availability
            )
        } else if b.rhs().span().contains_line_col(line_col) {
            jump_to_definition_in_type_expr_kind(
                schema,
                source,
                &b.rhs().kind,
                namespace_path,
                line_col,
                generics_declarations,
                availability
            )
        } else {
            vec![]
        }
        TypeExprKind::TypeGroup(g) => if g.type_expr().span().contains_line_col(line_col) {
            jump_to_definition_in_type_expr_kind(
                schema,
                source,
                &g.type_expr().kind,
                namespace_path,
                line_col,
                generics_declarations,
                availability
            )
        } else {
            vec![]
        }
        TypeExprKind::TypeTuple(type_tuple) => {
            for t in type_tuple.items() {
                if t.span().contains_line_col(line_col) {
                    return jump_to_definition_in_type_expr_kind(
                        schema,
                        source,
                        &t.kind,
                        namespace_path,
                        line_col,
                        generics_declarations,
                        availability
                    );
                }
            }
            vec![]
        }
        TypeExprKind::TypeSubscript(type_subscript) => if type_subscript.argument().span().contains_line_col(line_col) {
            return jump_to_definition_in_type_expr_kind(
                schema,
                source,
                &type_subscript.argument().kind,
                namespace_path,
                line_col,
                generics_declarations,
                availability
            );
        } else if type_subscript.container().span().contains_line_col(line_col) {
            jump_to_definition_in_type_expr_kind(
                schema,
                source,
                &type_subscript.container().kind,
                namespace_path,
                line_col,
                generics_declarations,
                availability
            )
        } else {
            vec![]
        }
        TypeExprKind::FieldName(_) => vec![],
        TypeExprKind::TypeItem(type_item) => jump_to_definition_in_type_item(
            schema,
            source,
            type_item,
            namespace_path,
            line_col,
            generics_declarations,
            availability
        ),
        TypeExprKind::TypedEnum(_) => vec![],
        TypeExprKind::TypedShape(typed_shape) => jump_to_definition_in_typed_shape(
            schema,
            source,
            typed_shape,
            namespace_path,
            line_col,
            generics_declarations,
            availability,
        )
    }
}

fn jump_to_definition_in_typed_shape(
    schema: &Schema,
    source: &Source,
    typed_shape: &TypedShape,
    namespace_path: &Vec<&str>,
    line_col: (usize, usize),
    generics_declarations: &Vec<&GenericsDeclaration>,
    availability: Availability,
) -> Vec<Definition> {
    for item in typed_shape.items() {
        if item.type_expr().span().contains_line_col(line_col) {
            return jump_to_definition_in_type_expr_kind(
                schema,
                source,
                &item.type_expr().kind,
                namespace_path,
                line_col,
                generics_declarations,
                availability,
            );
        }
    }
    vec![]
}

fn jump_to_definition_in_type_item(
    schema: &Schema,
    source: &Source,
    type_item: &TypeItem,
    namespace_path: &Vec<&str>,
    line_col: (usize, usize),
    generics_declarations: &Vec<&GenericsDeclaration>,
    availability: Availability,
) -> Vec<Definition> {
    for gen in type_item.generic_items() {
        if gen.span().contains_line_col(line_col) {
            return jump_to_definition_in_type_expr_kind(
                schema, 
                source,
                &gen.kind,
                namespace_path,
                line_col,
                generics_declarations,
                availability
            );
        }
    }
    if type_item.identifier_path().span.contains_line_col(line_col) {
        if type_item.identifier_path().identifiers.len() == 1 {
            let identifier = type_item.identifier_path().identifiers().next().unwrap();
            for generics_declaration in generics_declarations {
                if let Some(i) = generics_declaration.identifiers().find(|i| i.name() == identifier.name()) {
                    return vec![Definition {
                        path: schema.source(generics_declaration.source_id()).unwrap().file_path.clone(),
                        selection_span: identifier.span,
                        target_span: generics_declaration.span,
                        identifier_span: i.span,
                    }]
                }
            }
        }
        let mut user_typed_spaces = vec![];
        let mut selector_span = None;
        for identifier in type_item.identifier_path().identifiers() {
            if identifier.span.contains_line_col(line_col) {
                user_typed_spaces.push(identifier.name());
                selector_span = Some(identifier.span);
                break
            } else {
                user_typed_spaces.push(identifier.name());
            }
        }
        let reference = search_identifier_path_names_with_filter_to_path(&user_typed_spaces, schema, source, namespace_path, &top_filter_for_reference_type(ReferenceSpace::Default), availability);
        if let Some(reference) = reference {
            let top = schema.find_top_by_path(&reference).unwrap();
            return vec![Definition {
                path: schema.source(top.source_id()).unwrap().file_path.clone(),
                selection_span: selector_span.unwrap(),
                target_span: top.span(),
                identifier_span: top.identifier_span().unwrap(),
            }]
        }
    }
    vec![]
}