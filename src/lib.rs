macro_rules! test_parse {
    ($text:expr, $parse:expr) => {
        assert_eq!(parse_expr_str($text).unwrap(), $parse);
    };
}

mod expr;
mod parser;
mod solve;
mod substitute;

#[cfg(test)]
mod tests {
    use expr::{Expr, Operator};
    use parser::parse_expr_str;

    use super::*;

    fn int(i: i64) -> Expr {
        Expr::Integer(i)
    }

    fn var(name: &'static str) -> Expr {
        Expr::Variable(name.to_string())
    }

    fn add(a: Expr, b: Expr) -> Expr {
        Expr::Operation(Operator::Add, vec![a, b])
    }

    fn sub(a: Expr, b: Expr) -> Expr {
        Expr::Operation(Operator::Subtract, vec![a, b])
    }

    fn mul(a: Expr, b: Expr) -> Expr {
        Expr::Operation(Operator::Multiply, vec![a, b])
    }

    fn div(a: Expr, b: Expr) -> Expr {
        Expr::Operation(Operator::Divide, vec![a, b])
    }

    fn ln(expr: Expr) -> Expr {
        Expr::Operation(Operator::Ln, vec![expr])
    }

    #[test]
    fn parse() {
        test_parse!("   42\n\t ", int(42));
        test_parse!("  \t \n \t x  ", var("x"));
        test_parse!("(x)", var("x"));
        test_parse!("(\t107  ) ", int(107));
        test_parse!("1 +2 ", add(int(1), int(2)));
        test_parse!("5 +x ", add(int(5), var("x")));
        test_parse!(" (x )-  7", sub(var("x"), int(7)));
        test_parse!("105 * y", mul(int(105), var("y")));
        test_parse!("x / y", div(var("x"), var("y")));
        test_parse!("ln(x + 1)", ln(add(var("x"), int(1))));
    }
}
