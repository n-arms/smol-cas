pub type Name = String;

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Ln,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(Name),
    Integer(i64),
    Operation(Operator, Vec<Expr>),
}
