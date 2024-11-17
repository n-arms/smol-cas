use std::collections::HashMap;

use crate::expr::{Expr, Name};

impl Expr {
    pub fn substitute(self, name: Name, value: Expr) -> Expr {
        let mut map = HashMap::new();
        map.insert(name, value);
        self.substitute_all(&map)
    }

    pub fn substitute_all(self, map: &HashMap<Name, Expr>) -> Expr {
        match self {
            Expr::Variable(n) => {
                if let Some(e) = map.get(&n) {
                    e.clone()
                } else {
                    Expr::Variable(n.clone())
                }
            }
            Expr::Integer(int) => Expr::Integer(int),
            Expr::Operation(op, values) => Expr::Operation(
                op,
                values
                    .into_iter()
                    .map(|expr| expr.substitute_all(map))
                    .collect(),
            ),
        }
    }

    pub fn unify(self, other: Expr) -> Option<HashMap<Name, Expr>> {
        match (self, other) {
            (Expr::Integer(int1), Expr::Integer(int2)) => {
                if int1 == int2 {
                    Some(HashMap::new())
                } else {
                    None
                }
            }
            (Expr::Variable(var), e) => Some(HashMap::from([(var, e)])),
            (Expr::Operation(op1, vals1), Expr::Operation(op2, vals2)) if op1 == op2 => {
                if vals1.len() == vals2.len() {
                    let mut map = HashMap::new();
                    for (v1, v2) in vals1.into_iter().zip(vals2) {
                        map.extend(v1.unify(v2)?);
                    }
                    Some(map)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
