use crate::{declare_container_node, impl_container_node_defaults, node_child_fn, node_optional_child_fn};
use crate::ast::doc_comment::DocComment;
use crate::ast::identifier_path::IdentifierPath;
use crate::format::Writer;
use crate::traits::write::Write;

declare_container_node!(SynthesizedShapeFieldDeclaration, availability,
    pub(crate) comment: Option<usize>,
    pub(crate) decorator_identifier_path: usize,
);

impl_container_node_defaults!(SynthesizedShapeFieldDeclaration, availability);

impl SynthesizedShapeFieldDeclaration {

    node_optional_child_fn!(comment, DocComment);

    node_child_fn!(decorator_identifier_path, IdentifierPath);
}

impl Write for SynthesizedShapeFieldDeclaration {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}