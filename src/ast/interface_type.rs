use std::fmt::{Display, Formatter};
use crate::ast::arity::Arity;
use crate::ast::identifier_path::IdentifierPath;
use crate::ast::span::Span;

#[derive(Debug)]
pub(crate) struct InterfaceType {
    pub(crate) name: IdentifierPath,
    pub(crate) args: Vec<InterfaceType>,
    pub(crate) arity: Arity,
    pub(crate) collection_optional: bool,
    pub(crate) optional: bool,
    pub(crate) span: Span,
}

impl InterfaceType {

    // pub(crate) fn alter_generics_with(&self, map: &HashMap<String, InterfaceType>) -> Self {
    //     InterfaceType {
    //         name: self.name.alter_generics_with(map).name,
    //         args: self.args.iter().map(|arg| arg.alter_generics_with(map)).collect(),
    //         span: self.span.clone(),
    //         collection_optional: self.collection_optional,
    //         optional: self.optional,
    //         arity: self.arity,
    //     }
    // }
}

impl Display for InterfaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)?;
        if self.args.len() > 0 {
            f.write_str("<")?;
        }
        for (index, arg) in self.args.iter().enumerate() {
            Display::fmt(arg, f)?;
            if index != self.args.len() - 1 {
                f.write_str(", ")?;
            }
        }
        if self.args.len() > 0 {
            f.write_str(">")?;
        }
        if self.optional {
            f.write_str("?")?;
        }
        if self.arity != Arity::Scalar {
            match self.arity {
                Arity::Array => f.write_str("[]")?,
                Arity::Dictionary => f.write_str("{}")?,
                _ => ()
            };
            if self.collection_optional {
                f.write_str("?")?;
            }
        }
        Ok(())
    }
}