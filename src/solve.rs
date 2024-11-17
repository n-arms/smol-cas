use super::expr::*;

pub struct Rule {
    rule: Box<dyn Fn(Expr) -> Option<Expr>>,
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
        applications.push(expr);
        return applications;
    }
}

pub fn solve(expr: Expr, rules: &[Rule]) -> Vec<Expr> {
    let mut exprs = vec![expr];
    for _ in 0..100 {
        for rule in rules {
            exprs = exprs
                .into_iter()
                .flat_map(|e| rule.applications(e))
                .collect();
        }
    }
    return exprs;
}
