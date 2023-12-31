use crate::ast::arith_expr::{ArithExpr, BinaryOperation, ArithExprOperator, UnaryOperation, UnaryPostfixOperation};
use crate::{parse_container_node_variables_cleanup, parse_container_node_variables_without_span, parse_insert_operator};
use crate::parser::parse_expression::parse_expression;
use crate::parser::parse_span::parse_span;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, EXPR_PRATT_PARSER, Rule};
use crate::traits::identifiable::Identifiable;

pub(super) fn parse_arith_expr(pair: Pair<'_>, context: &ParserContext) -> ArithExpr {
    let span = parse_span(&pair);
    let result = EXPR_PRATT_PARSER.map_primary(|primary| match primary.as_rule() {
        Rule::operand => {
            let expression = parse_expression(primary, context);
            ArithExpr::Expression(Box::new(expression))
        },
        _ => {
            context.insert_unparsed(parse_span(&primary));
            unreachable!()
        },
    }).map_prefix(|operator, rhs| {
        let (path, mut children) = parse_container_node_variables_without_span!(context);
        let op = match operator.as_rule() {
            Rule::BI_NEG => ArithExprOperator::BitNeg,
            Rule::NEG => ArithExprOperator::Neg,
            Rule::NOT => ArithExprOperator::Not,
            _ => unreachable!(),
        };
        parse_insert_operator!(context, operator, children, operator.as_str());
        let rhs_id = rhs.id();
        children.insert(rhs_id, rhs.into());
        let operation = UnaryOperation {
            span,
            path,
            children,
            op,
            rhs: rhs_id,
        };
        parse_container_node_variables_cleanup!(context);
        ArithExpr::UnaryOperation(operation)
    }).map_infix(|lhs, operator, rhs| {
        let op = match operator.as_rule() {
            Rule::ADD => ArithExprOperator::Add,
            Rule::SUB => ArithExprOperator::Sub,
            Rule::MUL => ArithExprOperator::Mul,
            Rule::DIV => ArithExprOperator::Div,
            Rule::MOD => ArithExprOperator::Mod,
            Rule::BI_AND => ArithExprOperator::BitAnd,
            Rule::BI_XOR => ArithExprOperator::BitXor,
            Rule::BI_OR => ArithExprOperator::BitOr,
            Rule::NULLISH_COALESCING => ArithExprOperator::NullishCoalescing,
            Rule::BI_LS => ArithExprOperator::BitLS,
            Rule::BI_RS => ArithExprOperator::BitRS,
            Rule::AND => ArithExprOperator::And,
            Rule::OR => ArithExprOperator::Or,
            Rule::GT => ArithExprOperator::Gt,
            Rule::GTE => ArithExprOperator::Gte,
            Rule::LT => ArithExprOperator::Lt,
            Rule::LTE => ArithExprOperator::Lte,
            Rule::EQ => ArithExprOperator::Eq,
            Rule::NEQ => ArithExprOperator::Neq,
            Rule::RANGE_CLOSE => ArithExprOperator::RangeClose,
            Rule::RANGE_OPEN => ArithExprOperator::RangeOpen,
            _ => unreachable!(),
        };
        let (path, mut children) = parse_container_node_variables_without_span!(context);
        let lhs_id = lhs.id();
        let rhs_id = rhs.id();
        children.insert(lhs_id, lhs.into());
        parse_insert_operator!(context, operator, children, operator.as_str());
        children.insert(rhs_id, rhs.into());
        let operation = BinaryOperation {
            span,
            path,
            children,
            op,
            lhs: lhs_id,
            rhs: rhs_id,
        };
        parse_container_node_variables_cleanup!(context);
        ArithExpr::BinaryOperation(operation)
    }).map_postfix(|lhs, operator| {
        let op = match operator.as_rule() {
            Rule::FORCE_UNWRAP => ArithExprOperator::ForceUnwrap,
            _ => unreachable!(),
        };
        let (path, mut children) = parse_container_node_variables_without_span!(context);
        let lhs_id = lhs.id();
        children.insert(lhs_id, lhs.into());
        parse_insert_operator!(context, operator, children, operator.as_str());
        let operation = UnaryPostfixOperation {
            span,
            path,
            children,
            op,
            lhs: lhs_id,
        };
        parse_container_node_variables_cleanup!(context);
        ArithExpr::UnaryPostfixOperation(operation)
    }).parse(pair.into_inner());
    result
}