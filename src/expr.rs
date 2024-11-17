pub type Name = String;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Ln,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Variable(Name),
    Integer(i64),
    Operation(Operator, Vec<Expr>),
}
