use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use crate::ast::accessible::Accessible;
use crate::ast::argument_list::ArgumentList;
use crate::ast::arith::ArithExpr;
use crate::ast::group::Group;
use crate::ast::pipeline::Pipeline;
use crate::ast::identifier::Identifier;
use crate::ast::literals::{ArrayLiteral, BoolLiteral, DictionaryLiteral, EnumVariantLiteral, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::ast::span::Span;
use crate::ast::subscript::Subscript;
use crate::ast::unit::Unit;

#[derive(Debug)]
pub(crate) struct Negation {
    pub(crate) expression: Box<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for Negation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("-")?;
        Display::fmt(self.expression.as_ref(), f)?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct BitwiseNegation {
    pub(crate) expression: Box<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for BitwiseNegation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("~")?;
        Display::fmt(self.expression.as_ref(), f)?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct NullishCoalescing {
    pub(crate) expressions: Vec<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for NullishCoalescing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let len = self.expressions.len();
        for (index, expression) in self.expressions.iter().enumerate() {
            Display::fmt(expression, f)?;
            if index != len - 1 {
                f.write_str(" ?? ")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum ExpressionKind {
    Group(Group),
    NullishCoalescing(NullishCoalescing),
    Negation(Negation),
    BitwiseNegation(BitwiseNegation),
    ArithExpr(ArithExpr),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    RegExpLiteral(RegExpLiteral),
    BoolLiteral(BoolLiteral),
    NullLiteral(NullLiteral),
    EnumVariantLiteral(EnumVariantLiteral),
    RangeLiteral(RangeLiteral),
    TupleLiteral(TupleLiteral),
    ArrayLiteral(ArrayLiteral),
    DictionaryLiteral(DictionaryLiteral),
    Identifier(Identifier),
    ArgumentList(ArgumentList),
    Subscript(Subscript),
    Unit(Unit),
    Pipeline(Pipeline),
}

impl ExpressionKind {

    pub(crate) fn span(&self) -> &Span {
        match self {
            ExpressionKind::Group(e) => &e.span,
            ExpressionKind::NullishCoalescing(e) => &e.span,
            ExpressionKind::Negation(e) => &e.span,
            ExpressionKind::BitwiseNegation(e) => &e.span,
            ExpressionKind::ArithExpr(e) => e.span(),
            ExpressionKind::NumericLiteral(e) => &e.span,
            ExpressionKind::StringLiteral(e) => &e.span,
            ExpressionKind::RegExpLiteral(e) => &e.span,
            ExpressionKind::BoolLiteral(e) => &e.span,
            ExpressionKind::NullLiteral(e) => &e.span,
            ExpressionKind::EnumVariantLiteral(e) => &e.span,
            ExpressionKind::RangeLiteral(e) => &e.span,
            ExpressionKind::TupleLiteral(e) => &e.span,
            ExpressionKind::ArrayLiteral(e) => &e.span,
            ExpressionKind::DictionaryLiteral(e) => &e.span,
            ExpressionKind::Identifier(e) => &e.span,
            ExpressionKind::ArgumentList(e) => &e.span,
            ExpressionKind::Subscript(e) => &e.span,
            ExpressionKind::Unit(e) => &e.span,
            ExpressionKind::Pipeline(e) => &e.span,
        }
    }

    pub(crate) fn as_numeric_literal(&self) -> Option<&NumericLiteral> {
        match self {
            ExpressionKind::NumericLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_numeric_literal_mut(&mut self) -> Option<&mut NumericLiteral> {
        match self {
            ExpressionKind::NumericLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_numeric_literal(&self) -> bool {
        self.as_numeric_literal().is_some()
    }

    pub(crate) fn as_string_literal(&self) -> Option<&StringLiteral> {
        match self {
            ExpressionKind::StringLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_string_literal_mut(&mut self) -> Option<&mut StringLiteral> {
        match self {
            ExpressionKind::StringLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_string_literal(&self) -> bool {
        self.as_string_literal().is_some()
    }

    pub(crate) fn as_regexp_literal(&self) -> Option<&RegExpLiteral> {
        match self {
            ExpressionKind::RegExpLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_regexp_literal_mut(&mut self) -> Option<&mut RegExpLiteral> {
        match self {
            ExpressionKind::RegExpLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_regexp_literal(&self) -> bool {
        self.as_regexp_literal().is_some()
    }

    pub(crate) fn as_bool_literal(&self) -> Option<&BoolLiteral> {
        match self {
            ExpressionKind::BoolLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_bool_literal_mut(&mut self) -> Option<&mut BoolLiteral> {
        match self {
            ExpressionKind::BoolLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_bool_literal(&self) -> bool {
        self.as_bool_literal().is_some()
    }

    pub(crate) fn as_null_literal(&self) -> Option<&NullLiteral> {
        match self {
            ExpressionKind::NullLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_null_literal_mut(&mut self) -> Option<&mut NullLiteral> {
        match self {
            ExpressionKind::NullLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_null_literal(&self) -> bool {
        self.as_null_literal().is_some()
    }

    pub(crate) fn as_enum_variant_literal(&self) -> Option<&EnumVariantLiteral> {
        match self {
            ExpressionKind::EnumVariantLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_enum_variant_literal_mut(&mut self) -> Option<&mut EnumVariantLiteral> {
        match self {
            ExpressionKind::EnumVariantLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_enum_variant_literal(&self) -> bool {
        self.as_enum_variant_literal().is_some()
    }

    pub(crate) fn as_range_literal(&self) -> Option<&RangeLiteral> {
        match self {
            ExpressionKind::RangeLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_range_literal_mut(&mut self) -> Option<&mut RangeLiteral> {
        match self {
            ExpressionKind::RangeLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_tuple(&self) -> Option<&TupleLiteral> {
        match self {
            ExpressionKind::TupleLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_tuple_mut(&mut self) -> Option<&mut TupleLiteral> {
        match self {
            ExpressionKind::TupleLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_array_literal(&self) -> Option<&ArrayLiteral> {
        match self {
            ExpressionKind::ArrayLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_array_literal_mut(&mut self) -> Option<&mut ArrayLiteral> {
        match self {
            ExpressionKind::ArrayLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn is_array_literal(&self) -> bool {
        self.as_array_literal().is_some()
    }

    pub(crate) fn as_dictionary(&self) -> Option<&DictionaryLiteral> {
        match self {
            ExpressionKind::DictionaryLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_dictionary_mut(&mut self) -> Option<&mut DictionaryLiteral> {
        match self {
            ExpressionKind::DictionaryLiteral(n) => Some(n),
            _ => None,
        }
    }

    pub(crate) fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            ExpressionKind::Identifier(i) => Some(i),
            _ => None,
        }
    }

    pub(crate) fn as_unit(&self) -> Option<&Unit> {
        match self {
            ExpressionKind::Unit(u) => Some(u),
            _ => None,
        }
    }

    pub(crate) fn as_unit_mut(&mut self) -> Option<&mut Unit> {
        match self {
            ExpressionKind::Unit(u) => Some(u),
            _ => None,
        }
    }

    pub(crate) fn as_argument_list(&self) -> Option<&ArgumentList> {
        match self {
            ExpressionKind::ArgumentList(a) => Some(a),
            _ => None,
        }
    }

    pub(crate) fn as_argument_list_mut(&mut self) -> Option<&mut ArgumentList> {
        match self {
            ExpressionKind::ArgumentList(a) => Some(a),
            _ => None,
        }
    }

    pub(crate) fn as_subscript(&self) -> Option<&Subscript> {
        match self {
            ExpressionKind::Subscript(s) => Some(s),
            _ => None,
        }
    }

    pub(crate) fn as_subscript_mut(&mut self) -> Option<&mut Subscript> {
        match self {
            ExpressionKind::Subscript(s) => Some(s),
            _ => None,
        }
    }

    pub(crate) fn as_pipeline(&self) -> Option<&Pipeline> {
        match self {
            ExpressionKind::Pipeline(p) => Some(p),
            _ => None,
        }
    }

    pub(crate) fn as_pipeline_mut(&mut self) -> Option<&mut Pipeline> {
        match self {
            ExpressionKind::Pipeline(p) => Some(p),
            _ => None,
        }
    }
}

impl Display for ExpressionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionKind::Group(g) => Display::fmt(g, f),
            ExpressionKind::NullishCoalescing(n) => Display::fmt(n, f),
            ExpressionKind::Negation(n) => Display::fmt(n, f),
            ExpressionKind::BitwiseNegation(n) => Display::fmt(n, f),
            ExpressionKind::NumericLiteral(e) => Display::fmt(e, f),
            ExpressionKind::StringLiteral(s) => Display::fmt(s, f),
            ExpressionKind::RegExpLiteral(r) => Display::fmt(r, f),
            ExpressionKind::BoolLiteral(b) => Display::fmt(b, f),
            ExpressionKind::NullLiteral(n) => Display::fmt(n, f),
            ExpressionKind::EnumVariantLiteral(e) => Display::fmt(e, f),
            ExpressionKind::RangeLiteral(r) => Display::fmt(r, f),
            ExpressionKind::TupleLiteral(t) => Display::fmt(t, f),
            ExpressionKind::ArrayLiteral(a) => Display::fmt(a, f),
            ExpressionKind::DictionaryLiteral(d) => Display::fmt(d, f),
            ExpressionKind::Identifier(i) => Display::fmt(i, f),
            ExpressionKind::ArgumentList(a) => Display::fmt(a, f),
            ExpressionKind::Subscript(s) => Display::fmt(s, f),
            ExpressionKind::Unit(u) => Display::fmt(u, f),
            ExpressionKind::Pipeline(p) => Display::fmt(p, f),
            ExpressionKind::ArithExpr(a) => Display::fmt(a, f),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Expression {
    pub(crate) kind: ExpressionKind,
    pub(crate) resolved: RefCell<Option<Accessible>>,
}

impl Expression {
    pub(crate) fn new(kind: ExpressionKind) -> Self {
        Self { kind, resolved: RefCell::new(None) }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}