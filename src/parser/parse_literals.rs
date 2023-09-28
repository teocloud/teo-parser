use std::str::FromStr;
use snailquote::unescape;
use regex::Regex;
use teo_teon::value::Value;
use crate::ast::argument_list::ArgumentList;
use crate::ast::expr::ExpressionKind;
use crate::ast::literals::{ArrayLiteral, BoolLiteral, DictionaryLiteral, EnumVariantLiteral, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::parser::parse_span::parse_span;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};

pub(super) fn parse_string_literal(pair: &Pair<'_>) -> StringLiteral {
    StringLiteral {
        value: unescape(pair.as_str()).unwrap(),
        span: parse_span(&pair),
    }
}

pub(super) fn parse_null_literal(pair: &Pair<'_>) -> NullLiteral {
    NullLiteral { span: parse_span(&pair) }
}

pub(super) fn parse_bool_literal(pair: &Pair<'_>) -> BoolLiteral {
    BoolLiteral {
        span: parse_span(&pair),
        value: pair.as_str() == "true",
    }
}

pub(super) fn parse_regexp_literal(pair: &Pair<'_>, context: &mut ParserContext) -> RegExpLiteral {
    let span = parse_span(&pair);
    let mut value = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::regexp_content => match Regex::new(current.as_str()) {
                Ok(regexp) => value = Some(regexp),
                Err(err) => context.insert_error(span.clone(), "RegExpError: invalid regular expression"),
            },
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    RegExpLiteral {
        value: value.unwrap_or(Regex::new("").unwrap()),
        span,
    }
}

pub(super) fn parse_numeric_literal(pair: &Pair<'_>, _context: &mut ParserContext) -> NumericLiteral {
    let str_value = pair.as_str();
    NumericLiteral {
        span: parse_span(&pair),
        value: if str_value.contains(".") { // default to float64
            Value::F64(f64::from_str(&str_value).unwrap())
        } else if let Ok(i32v) = i32::from_str(str_value) {
            Value::I32(i32v)
        } else {
            Value::I64(i64::from_str(str_value).unwrap())
        }
    }
}

pub(super) fn parse_enum_variant_literal(pair: &Pair<'_>, context: &mut ParserContext) -> EnumVariantLiteral {
    let span = parse_span(&pair);
    let mut argument_list: Option<ArgumentList> = None;
    let mut value: Option<String> = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::identifier => value = Some(current.as_str().to_owned()),
            Rule::argument_list => argument_list = Some(parse_argument_list(current, context)),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    EnumVariantLiteral { span, value: value.unwrap(), argument_list }
}

pub(super) fn parse_array_literal(pair: &Pair<'_>, context: &mut ParserContext) -> ArrayLiteral {
    let span = parse_span(&pair);
    let mut expressions: Vec<ExpressionKind> = vec![];
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::expression => expressions.push(parse_expression_kind(current, context)),
            Rule::comment_block => (),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    ArrayLiteral { expressions, span }
}

pub(super) fn parse_tuple_literal(pair: &Pair<'_>, context: &mut ParserContext) -> TupleLiteral {
    let span = parse_span(&pair);
    let mut expressions: Vec<ExpressionKind> = vec![];
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::expression => expressions.push(parse_expression_kind(current, context)),
            Rule::comment_block => (),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    TupleLiteral { expressions, span }
}

pub(super) fn parse_dictionary_literal(pair: &Pair<'_>, context: &mut ParserContext) -> DictionaryLiteral {
    let span = parse_span(&pair);
    let mut expressions: Vec<(ExpressionKind, ExpressionKind)> = vec![];
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::named_expression => expressions.push(parse_named_expression(current, context)),
            Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE | Rule::comment_block => (),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    DictionaryLiteral { expressions, span }
}

fn parse_named_expression(pair: Pair<'_>, context: &mut ParserContext) -> (ExpressionKind, ExpressionKind) {
    let mut key = None;
    let mut value = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::expression => if key.is_none() {
                key = Some(parse_expression_kind(current, context));
            } else {
                value = Some(parse_expression_kind(current, context));
            },
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    return (key.unwrap(), value.unwrap())
}

pub(super) fn parse_range_literal(pair: &Pair<'_>, context: &mut ParserContext) -> RangeLiteral {
    let span = parse_span(&pair);
    let mut start = None;
    let mut end = None;
    let mut closed = false;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::range_end => if start.is_some() {
                start = Some(parse_range_end(current, context))
            } else {
                end = Some(parse_range_end(current, context))
            },
            Rule::RANGE_OPEN => closed = false,
            Rule::RANGE_CLOSE => closed = true,
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    RangeLiteral { span, start: start.unwrap(), closed, end: end.unwrap() }
}

fn parse_range_end(pair: Pair<'_>, context: &mut ParserContext) -> ExpressionKind {
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::numeric_literal => return ExpressionKind::NumericLiteral(parse_numeric_literal(&current, context)),
            Rule::unit_without_range_literal => return parse_unit(current, context),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    unreachable!()
}