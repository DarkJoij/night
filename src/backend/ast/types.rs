use crate::frontend::objects_driver::NightObject;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum ExpressionCallback {
    Void,
    Numeric(f64),
    Literal(String),
    Typed(NightObject)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Void,
    Unary,
    Binary,
    Numeric
}

// This should be checked later, as it
// may not be skipped by various linters.
impl Display for ExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum ExpressionResult {
    Void,
    Numeric(f64),
    Literal(String)
}

impl Display for ExpressionResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ExpressionResult::*;

        match self {
            Numeric(number) => write!(f, "{number}"),
            Literal(string) => write!(f, "{string}"),
            _ => <Self as Debug>::fmt(self, f)
        }
    }
}

pub trait DefaultExpression {
    fn execute_numeric(&self) -> f64;
    fn execute_unary(&self) -> f64;
    fn execute_binary(&self) -> f64;
    fn execute(&self) -> ExpressionResult;
}
