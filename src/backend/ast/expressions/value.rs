use crate::backend::ast::{
    DefaultExpression,
    Expression,
    ExpressionResult
};
use crate::{spawn_core_error, spawn_type_error};

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Void,
    Numeric(f64),
    Literal(String),
    Recursive(Expression)
}

impl Value {
    // Must be checked later.
    pub fn unwrap_to_f64(&self) -> f64 {
        match self {
            Value::Numeric(number) => *number, // Might be replaced with `.clone()`.
            _ => spawn_type_error!("Expected numeric expression instead of {self}.")
        }
    }

    pub fn unwrap_to_string(&self) -> String {
        match self {
            Value::Literal(string) => string.clone(),
            _ => spawn_type_error!("Expected string expression instead of {self}.")
        }
    }

    pub fn unwrap_to_expression(&self) -> Expression {
        match self {
            Value::Recursive(expression) => expression.clone(),
            _ => spawn_type_error!("Expected some expression instead of {self}.")
        }
    }

    pub fn get_f64_from_expr(&self) -> f64 {
        let expression = self.unwrap_to_expression();

        match expression.execute() {
            ExpressionResult::Numeric(number) => number,
            _ => spawn_core_error!("Failed to fetch numeric value: {self}.")
        }
    }

    #[cfg(feature = "daily")]
    pub fn get_string_from_expr(&self) -> String {
        let expression = self.unwrap_to_expression();

        match expression.execute() {
            ExpressionResult::Literal(string) => string,
            _ => spawn_core_error!("Failed to fetch string value: {self}.")
        }
    }

    #[cfg(feature = "daily")]
    pub fn get_child_from_expr(&self) -> Expression {
        let expression = self.unwrap_to_expression();

        match expression.execute() {
            // Must be modified for actual types.
            ExpressionResult::Some(child) => child,
            _ => spawn_core_error!("Failed to fetch expression: {self}.")
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Value::*;

        match self {
            Void => write!(f, "void"),
            Numeric(number) => write!(f, "{number}"),
            Literal(string) => write!(f, "{string}"),
            Recursive(expression) => write!(f, "{expression}")
        }
    }
}

#[cfg(tests)]
mod tests {
    use crate::backend::ast::Expression;
    use super::Value;

    #[test]
    fn test_methods() {
        let operand = Expression::numeric("100".to_owned());

        let number = Value::Numeric(1.);
        let string = Value::Literal("Hello world!".to_owned());
        let expression = Value::Recursive(operand.clone());

        assert_eq!(number.unwrap_to_f64(), 1.);
        assert_eq!(string.unwrap_to_string(), "Hello world!".to_owned());
        assert_eq!(expression.unwrap_to_expression(), operand);
    }

    #[test]
    fn test_display() {
        let void = Value::Void;
        let number = Value::Numeric(-1000.);
        let string = Value::Literal("Hello world!".to_owned());

        assert_eq!(format!("{void}"), "void".to_owned());
        assert_eq!(format!("{number}"), "-1000".to_owned());
        assert_eq!(format!("{string}"), "Hello world!".to_owned());
    }
}
