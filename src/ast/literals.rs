use std::fmt::{Display, Formatter};
use crate::ast::argument_list::ArgumentList;
use crate::ast::expr::ExpressionKind;
use crate::ast::span::Span;

#[derive(Debug)]
pub(crate) struct NumericLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}


#[derive(Debug)]
pub(crate) struct StringLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug)]
pub(crate) struct RegExpLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for RegExpLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug)]
pub(crate) struct BoolLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for BoolLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug)]
pub(crate) struct NullLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Display for NullLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

#[derive(Debug)]
pub(crate) struct EnumVariantLiteral {
    pub(crate) value: String,
    pub(crate) span: Span,
    pub(crate) argument_list: Option<ArgumentList>,
}

impl Display for EnumVariantLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")?;
        f.write_str(&self.value)?;
        if let Some(argument_list) = &self.argument_list {
            Display::fmt(argument_list, f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct RangeLiteral {
    pub(crate) closed: bool,
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for RangeLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(if self.closed { "..." } else { ".." })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct TupleLiteral {
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for TupleLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str(")")
    }
}

#[derive(Debug)]
pub(crate) struct ArrayLiteral {
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for ArrayLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str("]")
    }
}

#[derive(Debug)]
pub(crate) struct DictionaryLiteral {
    pub(crate) expressions: Vec<(ExpressionKind, ExpressionKind)>,
    pub(crate) span: Span,
}

impl Display for DictionaryLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        let len = self.expressions.len();
        for (index, (key, expression)) in self.expressions.iter().enumerate() {
            Display::fmt(key, f)?;
            f.write_str(": ")?;
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str("}")
    }
}
