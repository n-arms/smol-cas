use super::expr::*;
use chumsky::{
    error::Simple,
    primitive::just,
    recursive::recursive,
    text::{ident, int, keyword, whitespace},
    Parser,
};

pub fn parse_expr_str(text: &str) -> Result<Expr, Vec<Simple<char>>> {
    expr_parser().parse(text)
}

pub fn parse_expr(text: &[char]) -> Result<Expr, Vec<Simple<char>>> {
    expr_parser().parse(text)
}

fn expr_parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(ln_parser)
}

fn mul_div_parser(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, Expr, Error = Simple<char>> {
    whitespace()
        .ignore_then(terminal(expr.clone()))
        .then(just('*').or(just('/')).then(terminal(expr)).repeated())
        .foldl(|a, (op, b)| {
            if op == '*' {
                Expr::Operation(Operator::Multiply, vec![a, b])
            } else {
                Expr::Operation(Operator::Divide, vec![a, b])
            }
        })
        .then_ignore(whitespace())
}

fn add_sub_parser(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, Expr, Error = Simple<char>> {
    whitespace()
        .ignore_then(mul_div_parser(expr.clone()))
        .then(
            just('+')
                .or(just('-'))
                .then(mul_div_parser(expr))
                .repeated(),
        )
        .foldl(|a, (op, b)| {
            if op == '+' {
                Expr::Operation(Operator::Add, vec![a, b])
            } else {
                Expr::Operation(Operator::Subtract, vec![a, b])
            }
        })
        .then_ignore(whitespace())
}

fn ln_parser(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, Expr, Error = Simple<char>> {
    whitespace()
        .ignore_then(keyword("ln").ignore_then(expr.clone()))
        .then_ignore(whitespace())
        .map(|expr| Expr::Operation(Operator::Ln, vec![expr]))
        .or(add_sub_parser(expr))
}

fn terminal(
    expr: impl Parser<char, Expr, Error = Simple<char>> + Clone,
) -> impl Parser<char, Expr, Error = Simple<char>> {
    whitespace()
        .ignore_then(
            integer_expr()
                .or(variable_expr())
                .or(just('(').ignore_then(expr).then_ignore(just(')'))),
        )
        .then_ignore(whitespace())
}

fn integer_expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    whitespace()
        .ignore_then(int(10))
        .then_ignore(whitespace())
        .map(|int: String| Expr::Integer(int.parse().unwrap()))
}

fn variable_expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    whitespace()
        .ignore_then(ident())
        .then_ignore(whitespace())
        .map(|name| Expr::Variable(name))
}
