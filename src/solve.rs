use chumsky::error::Simple;

use crate::parser::parse_expr_str;

use super::expr::*;

pub fn rules<const N: usize>(
    rules: [(&'static str, &'static str); N],
) -> Result<Vec<Rule>, Vec<Simple<char>>> {
    rules
        .into_iter()
        .map(|(old, new)| Ok(Rule::new(parse_expr_str(old)?, parse_expr_str(new)?)))
        .collect()
}

pub struct Rule {
    pub rule: Box<dyn Fn(Expr) -> Option<Expr>>,
}

impl Rule {
    fn apply(&self, expr: Expr) -> Option<Expr> {
        self.rule.as_ref()(expr)
    }

    fn applications(&self, expr: Expr) -> Vec<Expr> {
        let mut applications: Vec<_> = self.apply(expr.clone()).into_iter().collect();
        if let Expr::Operation(op, values) = expr.clone() {
            for i in 0..values.len() {
                let apps = self.applications(values[i].clone());
                let prefix = values[0..i].to_vec();
                let suffix = values[i + 1..].to_vec();

                for app in apps {
                    let mut new_values = Vec::new();
                    new_values.extend(prefix.clone());
                    new_values.push(app);
                    new_values.extend(suffix.clone());
                    applications.push(Expr::Operation(op, new_values));
                }
            }
        }
        return applications;
    }

    fn new(old: Expr, new: Expr) -> Self {
        Self {
            rule: Box::new(move |e: Expr| {
                let sub = Expr::unify(old.clone(), e)?;
                Some(new.clone().substitute_all(&sub))
            }),
        }
    }
}

pub fn solve(expr: Expr, rules: &[Rule]) -> Vec<Expr> {
    let mut exprs = vec![expr.clone()];
    let mut to_solve = vec![expr];
    for i in 0..3 {
        let mut new_to_solve = Vec::new();
        println!("on solve iter {}", i);
        for rule in rules {
            println!("  testing rule with {} expressions", exprs.len());
            new_to_solve.extend(
                to_solve
                    .clone()
                    .into_iter()
                    .flat_map(|e| rule.applications(e)),
            );
        }
        to_solve = new_to_solve;
        exprs.extend(to_solve.clone());
    }
    return exprs;
}
