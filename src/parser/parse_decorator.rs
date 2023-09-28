use std::cell::RefCell;
use crate::ast::decorator::Decorator;
use crate::ast::expr::ExpressionKind;
use crate::parser::parse_span::parse_span;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};

pub(super) fn parse_decorator(pair: Pair<'_>, context: &mut ParserContext) -> Decorator {
    let span = parse_span(&pair);
    let mut unit: Option<ExpressionKind> = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::identifier_unit => unit = Some(parse_unit(current, context)),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    Decorator { span, expression: unit.unwrap(), resolved: RefCell::new(None) }
}