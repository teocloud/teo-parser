use crate::ast::argument_list::ArgumentList;
use crate::availability::Availability;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::ast::subscript::Subscript;
use crate::ast::unit::Unit;

pub fn search_unit_for_auto_completion<HAL, HS, HI, OUTPUT>(
    _schema: &Schema,
    _source: &Source,
    _unit: &Unit,
    _namespace_path: &Vec<&str>,
    _line_col: (usize, usize),
    _handle_argument_list: HAL,
    _handle_subscript: HS,
    _handle_identifier: HI,
    default: OUTPUT,
    _availability: Availability,
) -> OUTPUT where
    HAL: Fn(&ArgumentList, &Vec<Vec<&str>>) -> OUTPUT,
    HS: Fn(&Subscript) -> OUTPUT,
    HI: Fn(&Vec<&str>, Option<&Vec<usize>>) -> OUTPUT,
{
    // let mut current: Option<UnitSearchResult> = None;
    // for (index, expression) in unit.expressions().enumerate() {
    //     if index == 0 {
    //         current = Some(if let Some(identifier) = expression.kind.as_identifier() {
    //             if expression.span().contains_line_col(line_col) {
    //                 return handle_identifier(&vec![], None);
    //             }
    //             if let Some(path) = search_identifier_path_names_with_filter(
    //                 schema,
    //                 source,
    //                 namespace_path,
    //                 &vec![identifier.name()],
    //                 &top_filter_for_reference_type(ReferenceType::Default),
    //                 availability,
    //             ) {
    //                 UnitSearchResult::Reference(path)
    //             } else {
    //                 UnitSearchResult::Type(Type::Undetermined)
    //             }
    //         } else {
    //             if expression.span().contains_line_col(line_col) {
    //                 return default;
    //             }
    //             UnitSearchResult::Type(expression.resolved().r#type().clone())
    //         });
    //         if current.is_some() && current.as_ref().unwrap().is_reference() {
    //             let top = schema.find_top_by_path(current.as_ref().unwrap().as_reference().unwrap()).unwrap();
    //             if top.is_constant() {
    //                 current = Some(UnitSearchResult::Type(top.as_constant().unwrap().resolved().expression_resolved.r#type.clone()));
    //             }
    //         }
    //     } else {
    //         let contains = expression.span().contains_line_col(line_col);
    //         if current.as_ref().is_some() {
    //             match current.as_ref().unwrap() {
    //                 UnitSearchResult::Type(current_type) => {
    //                     if let Some((path, _)) = current_type.as_struct_object() {
    //                         match &expression.kind {
    //                             ExpressionKind::Identifier(_) => {
    //                                 if contains {
    //                                     return handle_identifier(&vec![], Some(path));
    //                                 }
    //                                 return default
    //                             }
    //                             ExpressionKind::Call(call) => {
    //                                 let struct_declaration = schema.find_top_by_path(path).unwrap().as_struct_declaration().unwrap();
    //                                 if let Some(function_declaration) = struct_declaration.function_declarations.iter().find(|f| {
    //                                     f.r#static == false && f.identifier().name() == call.identifier().name()
    //                                 }) {
    //                                     if call.identifier().span.contains_line_col(line_col) {
    //                                         return handle_identifier(&vec![], Some(path));
    //                                     } else if call.argument_list.span.contains_line_col(line_col) {
    //                                         return handle_argument_list(&call.argument_list, &struct_declaration.path, Some(call.identifier().name()));
    //                                     } else {
    //                                         // going next
    //                                         current = Some(UnitSearchResult::Type(function_declaration.return_type().resolved().clone()));
    //                                     }
    //                                 } else {
    //                                     return default;
    //                                 }
    //                             }
    //                             ExpressionKind::Subscript(subscript) => {
    //                                 let struct_declaration = schema.find_top_by_path(path).unwrap().as_struct_declaration().unwrap();
    //                                 if subscript.span.contains_line_col(line_col) {
    //                                     if subscript.expression().span().contains_line_col(line_col) {
    //                                         return handle_subscript(&subscript);
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 } else {
    //                                     if let Some(subscript_function) = struct_declaration.function_declarations.iter().find(|f| {
    //                                         f.r#static == false && f.identifier().name() == "subscript"
    //                                     }) {
    //                                         current = Some(UnitSearchResult::Type(subscript_function.return_type().resolved().clone()));
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 }
    //                             }
    //                             _ => unreachable!(),
    //                         }
    //                     } else {
    //                         return default;
    //                     }
    //                 }
    //                 UnitSearchResult::Reference(current_reference) => {
    //                     match schema.find_top_by_path(&current_reference).unwrap() {
    //                         Node::StructDeclaration(struct_declaration) => {
    //                             match &expression.kind {
    //                                 ExpressionKind::ArgumentList(argument_list) => {
    //                                     if let Some(new) = struct_declaration.function_declarations.iter().find(|f| f.r#static && f.identifier().name() == "new") {
    //                                         if argument_list.span.contains_line_col(line_col) {
    //                                             return handle_argument_list(argument_list, &struct_declaration.path, Some(new.identifier().name()));
    //                                         } else {
    //                                             current = Some(UnitSearchResult::Type(new.return_type().resolved().clone()));
    //                                         }
    //                                     } else {
    //                                         return handle_argument_list(argument_list, &struct_declaration.path, Some(new.identifier().name()));
    //                                     }
    //                                 }
    //                                 ExpressionKind::Call(call) => {
    //                                     if let Some(function) = struct_declaration.function_declarations.iter().find(|f| f.r#static && f.identifier().name() == call.identifier().name()) {
    //                                         if call.span.contains_line_col(line_col) {
    //                                             return handle_identifier(call.identifier().span, struct_declaration.path.as_ref(), Some(function.identifier().name()));
    //                                         } else if call.argument_list.span.contains_line_col(line_col) {
    //                                             return handle_argument_list(&call.argument_list, struct_declaration.path.as_ref(), Some(function.identifier().name()));
    //                                         } else {
    //                                             return default;
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 }
    //                                 ExpressionKind::Subscript(s) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Identifier(i) => {
    //                                     return default;
    //                                 }
    //                                 _ => unreachable!()
    //                             }
    //                         },
    //                         Node::Config(config) => {
    //                             match &expression.kind {
    //                                 ExpressionKind::Identifier(identifier) => {
    //                                     if let Some(item) = config.items().find(|i| i.identifier().name() == identifier.name()) {
    //                                         if identifier.span.contains_line_col(line_col) {
    //                                             return handle_identifier(identifier.span, config.path.as_ref(), Some(item.identifier().name()));
    //                                         } else {
    //                                             current = Some(UnitSearchResult::Type(item.expression.resolved().r#type.clone()));
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 },
    //                                 ExpressionKind::ArgumentList(a) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Call(c) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Subscript(s) => {
    //                                     return default;
    //                                 }
    //                                 _ => unreachable!()
    //                             }
    //                         }
    //                         Node::Enum(r#enum) => {
    //                             match &expression.kind {
    //                                 ExpressionKind::Identifier(i) => {
    //                                     if let Some(member) = r#enum.members().find(|m| m.identifier().name() == i.name()) {
    //                                         if i.span.contains_line_col(line_col) {
    //                                             return handle_identifier(i.span, r#enum.path.as_ref(), Some(member.identifier().name()));
    //                                         } else {
    //                                             return default;
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 }
    //                                 ExpressionKind::Call(c) => {
    //                                     if c.span.contains_line_col(line_col) {
    //                                         if let Some(member) = r#enum.members().find(|m| m.identifier().name() == c.identifier().name()) {
    //                                             if c.identifier().span.contains_line_col(line_col) {
    //                                                 return handle_identifier(c.identifier().span, r#enum.path.as_ref(), Some(member.identifier().name()));
    //                                             } else if c.argument_list.span.contains_line_col(line_col) {
    //                                                 return handle_argument_list(&c.argument_list, r#enum.path.as_ref(), Some(member.identifier().name()));
    //                                             } else {
    //                                                 return default;
    //                                             }
    //                                         } else {
    //                                             return default;
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 }
    //                                 ExpressionKind::ArgumentList(a) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Subscript(s) => {
    //                                     return default;
    //                                 }
    //                                 _ => unreachable!()
    //                             }
    //                         }
    //                         Node::Model(model) => {
    //                             match &expression.kind {
    //                                 ExpressionKind::Identifier(identifier) => {
    //                                     if let Some(field) = model.fields().find(|f| f.name() == identifier.name()) {
    //                                         if identifier.span.contains_line_col(line_col) {
    //                                             return handle_identifier(identifier.span, model.path.as_ref(), Some(field.name()));
    //                                         } else {
    //                                             return default;
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 },
    //                                 ExpressionKind::ArgumentList(a) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Call(c) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Subscript(s) => {
    //                                     return default;
    //                                 }
    //                                 _ => unreachable!()
    //                             }
    //                         }
    //                         Node::InterfaceDeclaration(interface) => {
    //                             match &expression.kind {
    //                                 ExpressionKind::Identifier(identifier) => {
    //                                     if let Some(field) = interface.fields().find(|f| f.name() == identifier.name()) {
    //                                         if identifier.span.contains_line_col(line_col) {
    //                                             return handle_identifier(identifier.span, interface.path.as_ref(), Some(field.name()));
    //                                         } else {
    //                                             return default;
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 },
    //                                 ExpressionKind::ArgumentList(a) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Call(c) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Subscript(s) => {
    //                                     return default;
    //                                 }
    //                                 _ => unreachable!()
    //                             }
    //                         }
    //                         Node::Namespace(namespace) => {
    //                             match &expression.kind {
    //                                 ExpressionKind::Identifier(identifier) => {
    //                                     if let Some(top) = namespace.find_top_by_name(identifier.name(), &top_filter_for_reference_type(ReferenceType::Default), availability) {
    //                                         if identifier.span.contains_line_col(line_col) {
    //                                             return handle_identifier(identifier.span, top.path(), None);
    //                                         } else {
    //                                             return default;
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 },
    //                                 ExpressionKind::Call(c) => {
    //                                     if let Some(top) = namespace.find_top_by_name(c.identifier().name(), &top_filter_for_reference_type(ReferenceType::Default), availability) {
    //                                         match top {
    //                                             Node::StructDeclaration(struct_declaration) => {
    //                                                 if let Some(new) = struct_declaration.function_declarations.iter().find(|f| {
    //                                                     f.identifier().name() == "new"
    //                                                 }) {
    //                                                     if c.span.contains_line_col(line_col) {
    //                                                         if c.identifier().span.contains_line_col(line_col) {
    //                                                             return handle_identifier(c.identifier().span, struct_declaration.path.as_ref(), Some("new"));
    //                                                         } else if c.argument_list.span.contains_line_col(line_col) {
    //                                                             return handle_argument_list(&c.argument_list, struct_declaration.path.as_ref(), Some("new"));
    //                                                         } else {
    //                                                             return default;
    //                                                         }
    //                                                     } else {
    //                                                         current = Some(UnitSearchResult::Type(new.return_type().resolved().clone()));
    //                                                     }
    //                                                 } else {
    //                                                     return default;
    //                                                 }
    //                                             },
    //                                             _ => return default,
    //                                         }
    //                                     } else {
    //                                         return default;
    //                                     }
    //                                 }
    //                                 ExpressionKind::ArgumentList(a) => {
    //                                     return default;
    //                                 }
    //                                 ExpressionKind::Subscript(s) => {
    //                                     return default;
    //                                 }
    //                                 _ => unreachable!()
    //                             }
    //                         }
    //                         _ => unreachable!()
    //                     }
    //                 }
    //             }
    //         } else {
    //             return default
    //         }
    //     }
    // }
    default
}
