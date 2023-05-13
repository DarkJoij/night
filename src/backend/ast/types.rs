use crate::spawn_type_error;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum ExpressionType {
    Void,
    Numeric,
    Literal
}

#[derive(Debug)]
pub enum ExpressionResult {
    Void,
    Numeric(f64),
    Literal(String)
}

impl ExpressionResult {
    pub fn unwrap_to_f64(&self) -> f64 {
        match self {
            // Might be replaced with `.clone()` calling.
            ExpressionResult::Numeric(number) => *number,
            _ => spawn_type_error!("Expected numeric expression instead of {self:?}")
        }
    }

    pub fn unwrap_to_string(&self) -> String {
        match self {
            ExpressionResult::Literal(string) => string.clone(),
            _ => spawn_type_error!("Expected string expression instead of {self:?}")
        }
    }
}

impl Display for ExpressionResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ExpressionResult::*;

        match self {
            Void => write!(f, "()"),
            Numeric(number) => write!(f, "{number}"),
            Literal(string) => write!(f, "{string}")
        }
    }
}
