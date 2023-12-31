use maplit::btreemap;
use crate::ast::argument_declaration::{ArgumentDeclaration};
use crate::ast::argument_list_declaration::ArgumentListDeclaration;
use crate::availability::Availability;
use crate::ast::generics::{GenericsConstraint, GenericsDeclaration};
use crate::resolver::resolve_type_expr::resolve_type_expr;
use crate::resolver::resolver_context::ResolverContext;

pub(super) fn resolve_argument_list_declaration<'a>(
    argument_list_declaration: &'a ArgumentListDeclaration,
    generics_declaration: &Vec<&'a GenericsDeclaration>,
    generics_constraint: &Vec<&'a GenericsConstraint>,
    context: &'a ResolverContext<'a>,
    availability: Availability,
) {
    for partial_argument_declaration in argument_list_declaration.partial_argument_declarations() {
        context.insert_diagnostics_error(partial_argument_declaration.span, "partial argument declaration");
    }
    for argument_declaration in argument_list_declaration.argument_declarations() {
        resolve_argument_declaration(argument_declaration, generics_declaration, generics_constraint, context, availability)
    }
}

fn resolve_argument_declaration<'a>(
    argument_declaration: &'a ArgumentDeclaration,
    generics_declaration: &Vec<&'a GenericsDeclaration>,
    generics_constraint: &Vec<&'a GenericsConstraint>,
    context: &'a ResolverContext<'a>,
    availability: Availability,
) {
    let result = resolve_type_expr(argument_declaration.type_expr(), generics_declaration, generics_constraint, &btreemap! {}, context, availability);
}