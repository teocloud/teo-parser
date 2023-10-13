use std::sync::Arc;
use crate::ast::decorator::Decorator;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::ast::top::Top;
use crate::definition::definition::Definition;
use crate::search::search_identifier_path::search_identifier_path_in_source;

pub(super) fn jump_to_definition_in_decorator<'a>(
    schema: &'a Schema,
    source: &'a Source,
    decorator: &'a Decorator,
    namespace_path: &Vec<&'a str>,
    line_col: (usize, usize),
    filter: &Arc<dyn Fn(&Top) -> bool>,
) -> Vec<Definition> {
    let mut user_typed_spaces = vec![];
    let mut selector_span = None;
    for identifier in decorator.identifier_path.identifiers.iter() {
        if identifier.span.contains_line_col(line_col) {
            user_typed_spaces.push(identifier.name());
            selector_span = Some(identifier.span);
            break
        } else {
            user_typed_spaces.push(identifier.name());
        }
    }
    let reference = search_identifier_path_in_source(schema, source, namespace_path, &user_typed_spaces, filter);
    match reference {
        Some(path) => {
            let top = schema.find_top_by_path(&path).unwrap();
            vec![Definition {
                path: schema.source(*path.get(0).unwrap()).unwrap().file_path.clone(),
                selection_span: selector_span.unwrap(),
                target_span: top.span(),
                identifier_span: match top {
                    Top::DecoratorDeclaration(d) => d.identifier.span,
                    Top::Namespace(n) => n.span,
                    _ => unreachable!()
                }
            }]
        },
        None => {
            vec![]
        }
    }
}