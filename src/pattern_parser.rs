macro_rules! parser {
    () => {
        impl Parser<char, Box<dyn Fn(Expr) -> Option<MatchResult>>, Error = Simple<char>> + Clone
    }
}

#[derive(Default)]
pub struct MatchResult {
    any: HashMap<Name, Expr>,
    ints: HashMap<Name, i64>,
}

impl MatchResult {
    fn from_int(name: Name, int: i64) -> Self {
        Self {
            ints: HashMap::from([(name, int)]),
            ..Default::default()
        }
    }

    fn from_any(name: Name, expr: Expr) -> MatchResult {
        Self {
            any: HashMap::from([(name, expr)]),
            ..Default::default()
        }
    }

    fn union(mut self, other: Self) -> Option<Self> {
        for (name, new_expr) in other.any {
            if let Some(old_expr) = self.any.get(&name) {
                if &new_expr != old_expr {
                    return None;
                }
            } else {
                self.any.insert(name, new_expr);
            }
        }
        for (name, new_int) in other.ints {
            if let Some(old_int) = self.ints.get(&name) {
                if new_int != *old_int {
                    return None;
                }
            } else {
                self.ints.insert(name, new_int);
            }
        }
        Some(self)
    }
}

use std::collections::HashMap;

use crate::expr::{Expr, Name, Operator};
use chumsky::{
    error::Simple,
    primitive::just,
    text::{ident, int, keyword, whitespace},
    Parser,
};

fn integer_pattern() -> parser!() {
    whitespace()
        .ignore_then(int(10))
        .then_ignore(whitespace())
        .map(|int: String| {
            let int1 = int.parse::<i64>().unwrap();
            Box::new(move |e: Expr| -> Option<MatchResult> {
                if let Expr::Integer(int_real) = e {
                    if int1 == int_real {
                        Some(MatchResult::default())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }) as Box<dyn Fn(Expr) -> Option<MatchResult>>
        })
}

fn int_var_pattern() -> parser!() {
    whitespace()
        .ignore_then(just('#'))
        .ignore_then(ident())
        .then_ignore(whitespace())
        .map(|var| {
            Box::new(move |e| {
                if let Expr::Integer(int) = e {
                    Some(MatchResult::from_int(var.clone(), int))
                } else {
                    None
                }
            }) as Box<dyn Fn(Expr) -> Option<MatchResult>>
        })
}

fn any_var_pattern() -> parser!() {
    whitespace()
        .ignore_then(just('@'))
        .ignore_then(ident())
        .then_ignore(whitespace())
        .map(|var| {
            Box::new(move |e| Some(MatchResult::from_any(var.clone(), e)))
                as Box<dyn Fn(Expr) -> Option<MatchResult>>
        })
}

fn terminal(expr: parser!()) -> parser!() {
    whitespace()
        .ignore_then(
            integer_pattern().or(int_var_pattern()
                .or(any_var_pattern().or(just('(').ignore_then(expr).then_ignore(just(')'))))),
        )
        .then_ignore(whitespace())
}

fn mul_div_parser(expr: parser!()) -> parser!() {
    whitespace()
        .ignore_then(terminal(expr.clone()))
        .then(just('*').or(just('/')).then(terminal(expr)).repeated())
        .foldl(|a, (op, b)| {
            let op = if op == '*' {
                Operator::Multiply
            } else {
                Operator::Divide
            };
            Box::new(move |e| {
                if let Expr::Operation(real_op, args) = e {
                    if real_op == op {
                        let a_match = a.as_ref()(args[0].clone())?;
                        let b_match = b.as_ref()(args[1].clone())?;
                        return a_match.union(b_match);
                    }
                }
                None
            })
        })
        .then_ignore(whitespace())
}

fn add_sub_parser(expr: parser!()) -> parser!() {
    whitespace()
        .ignore_then(mul_div_parser(expr.clone()))
        .then(
            just('+')
                .or(just('-'))
                .then(mul_div_parser(expr))
                .repeated(),
        )
        .foldl(|a, (op, b)| {
            let op = if op == '+' {
                Operator::Add
            } else {
                Operator::Subtract
            };
            Box::new(move |e| {
                if let Expr::Operation(real_op, args) = e {
                    if real_op == op {
                        let a_match = a.as_ref()(args[0].clone())?;
                        let b_match = b.as_ref()(args[1].clone())?;
                        return a_match.union(b_match);
                    }
                }
                None
            })
        })
        .then_ignore(whitespace())
}

fn ln_parser(expr: parser!()) -> parser!() {
    whitespace()
        .ignore_then(keyword("ln").ignore_then(expr.clone()))
        .then_ignore(whitespace())
        .map(|pattern| {
            Box::new(move |expr| {
                if let Expr::Operation(Operator::Ln, args) = expr {
                    pattern.as_ref()(args[0].clone())
                } else {
                    None
                }
            }) as Box<dyn Fn(Expr) -> Option<MatchResult>>
        })
        .or(add_sub_parser(expr))
}
