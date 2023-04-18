use crate::frontend::objects_driver::NightObject;

#[derive(Debug, PartialEq)] // TODO: Replace me with `Display`.
pub enum ExpressionType {
    Unary,
    Binary,
    Numeric,
    Recursive
}

pub enum StatementType {
    Assignment,
    Println,
    Print
}

#[derive(Debug)]
pub enum ExpressionCallback {
    Void,
    Numeric(f64),
    Literal(String),
    Typed(NightObject)
}

pub trait NumericExpression<T> {
    fn execute_numeric(&self) -> T;
}

pub trait UnaryExpression<T> {
    fn execute_unary(&self) -> T;
}

pub trait BinaryExpression<T> {
    fn execute_binary(&self) -> T;
}

pub trait DefaultExpression {
    fn execute(&self) -> ExpressionCallback;
}
