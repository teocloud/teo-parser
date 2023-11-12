use crate::{declare_node, impl_node_defaults_with_display};

declare_node!(Identifier, name: String);

impl Identifier {

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl_node_defaults_with_display!(Identifier, name);
