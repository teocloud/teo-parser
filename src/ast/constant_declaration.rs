use std::cell::{Cell, RefCell};
use crate::ast::expression::Expression;
use crate::ast::identifier::Identifier;
use crate::ast::type_expr::TypeExpr;
use crate::{declare_container_node, impl_container_node_defaults, node_child_fn, node_optional_child_fn};
use crate::ast::doc_comment::DocComment;
use crate::format::Writer;
use crate::traits::has_availability::HasAvailability;
use crate::traits::info_provider::InfoProvider;
use crate::traits::resolved::Resolve;
use crate::traits::write::Write;
use crate::expr::ExprInfo;

declare_container_node!(ConstantDeclaration, named, availability,
    pub(crate) comment: Option<usize>,
    pub(crate) identifier: usize,
    pub(crate) type_expr: Option<usize>,
    pub(crate) expression: usize,
    pub(crate) use_count: Cell<usize>,
    pub(crate) resolved: RefCell<Option<ExprInfo >>,
);

impl_container_node_defaults!(ConstantDeclaration, named, availability);

impl ConstantDeclaration {

    node_child_fn!(identifier, Identifier);

    node_optional_child_fn!(comment, DocComment);

    node_optional_child_fn!(type_expr, TypeExpr);

    node_child_fn!(expression, Expression);
}

impl InfoProvider for ConstantDeclaration {
    fn namespace_skip(&self) -> usize {
        1
    }
}

impl Resolve<ExprInfo> for ConstantDeclaration {
    fn resolved_ref_cell(&self) -> &RefCell<Option<ExprInfo>> {
        &self.resolved
    }
}

impl Write for ConstantDeclaration {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}