use crate::ast::doc_comment::DocComment;
use crate::ast::field::Field;
use crate::ast::identifier::Identifier;
use crate::{declare_container_node, impl_container_node_defaults, node_child_fn, node_children_iter, node_children_iter_fn, node_optional_child_fn};
use crate::ast::partial_field::PartialField;
use crate::format::Writer;
use crate::traits::has_availability::HasAvailability;
use crate::traits::info_provider::InfoProvider;
use crate::traits::named_identifiable::NamedIdentifiable;
use crate::traits::node_trait::NodeTrait;
use crate::traits::write::Write;

declare_container_node!(ConfigDeclaration, named, availability,
    pub(crate) comment: Option<usize>,
    pub(crate) identifier: usize,
    pub(crate) fields: Vec<usize>,
    pub(crate) partial_fields: Vec<usize>,
);

impl_container_node_defaults!(ConfigDeclaration, named, availability);

node_children_iter!(ConfigDeclaration, Field, FieldsIter, fields);

node_children_iter!(ConfigDeclaration, PartialField, PartialFieldsIter, partial_fields);

impl ConfigDeclaration {

    node_optional_child_fn!(comment, DocComment);

    node_child_fn!(identifier, Identifier);

    node_children_iter_fn!(fields, FieldsIter);

    node_children_iter_fn!(partial_fields, PartialFieldsIter);

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields().find(|f| f.identifier().name() == name)
    }
}

impl InfoProvider for ConfigDeclaration {

    fn namespace_skip(&self) -> usize {
        1
    }
}

impl Write for ConfigDeclaration {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}