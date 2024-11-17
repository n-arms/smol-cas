use crate::expr::{Expr, Name};

impl Expr {
    pub fn substitute(self, name: &Name, value: Expr) -> Expr {
        match self {
            Expr::Variable(n) => {
                if &n == name {
                    value
                } else {
                    Expr::Variable(n.clone())
                }
            }
            Expr::Integer(int) => Expr::Integer(int),
            Expr::Operation(op, values) => Expr::Operation(
                op,
                values
                    .into_iter()
                    .map(|expr| expr.substitute(name, value.clone()))
                    .collect(),
            ),
        }
    }
}
