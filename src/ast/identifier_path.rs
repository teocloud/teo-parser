use crate::{declare_container_node, impl_container_node_defaults, node_children_iter, node_children_iter_fn, node_optional_child_fn};
use crate::ast::empty_dot::EmptyDot;
use crate::format::Writer;
use crate::traits::node_trait::NodeTrait;
use crate::traits::write::Write;
use super::identifier::Identifier;

declare_container_node!(IdentifierPath, pub(crate) identifiers: Vec<usize>, pub(crate) empty_dot: Option<usize>);

impl_container_node_defaults!(IdentifierPath);

node_children_iter!(IdentifierPath, Identifier, IdentifiersIter, identifiers);

impl IdentifierPath {

    node_optional_child_fn!(empty_dot, EmptyDot);

    node_children_iter_fn!(identifiers, IdentifiersIter);

    pub fn names(&self) -> Vec<&str> {
        self.identifiers().map(|i| i.name()).collect()
    }
}

impl Write for IdentifierPath {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values())
    }
}
