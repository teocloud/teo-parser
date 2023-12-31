
use crate::{declare_node, impl_node_defaults};
use crate::format::Writer;
use crate::traits::write::Write;

declare_node!(AvailabilityFlagEnd);

impl_node_defaults!(AvailabilityFlagEnd);

impl Write for AvailabilityFlagEnd {

    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_content(self, "#end\n");
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}
