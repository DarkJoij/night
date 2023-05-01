use crate::backend::ast::{
    DefaultExpression,
    ExpressionResult,
    ExpressionType,

};
use crate::backend::ast::expressions::Value;

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Expression {
    left: Box<Value>,
    right: Box<Value>,
    operator: String,
    expression_type: ExpressionType
}

impl Expression {
    pub fn new(
        left: Value,
        right: Value,
        operator: String,
        expression_type: ExpressionType
    ) -> Self {
        Expression {
            left: Box::new(left), // Might be replaced with `box x` if i'll switch to night builds.
            right: Box::new(right),
            operator,
            expression_type
        }
    }

    pub fn numeric(value: String) -> Self {
        Self::new(
            Value::Literal(value),
            Value::Void,
            String::new(),
            ExpressionType::Numeric
        )
    }

    pub fn unary(operand: Expression, operator: String) -> Self {
        Self::new(
            Value::Recursive(operand),
            Value::Void,
            operator,
            ExpressionType::Unary
        )
    }

    pub fn binary(left: Expression, right: Expression, operator: String) -> Self {
        Self::new(
            Value::Recursive(left),
            Value::Recursive(right),
            operator,
            ExpressionType::Binary
        )
    }
}

impl DefaultExpression for Expression {
    fn execute_numeric(&self) -> f64 {
        let string = self.left.unwrap_to_string();

        match string.parse::<f64>() {
            Ok(n) => n,
            // TODO: Switch to custom errors.
            Err(_) => panic!("failed to parse string: {}.", &self.left)
        }
    }

    fn execute_unary(&self) -> f64 {
        let operand = self.left.unwrap_to_f64();

        match self.operator.as_str() {
            "-" => -operand,
            _ => operand
        }
    }

    fn execute_binary(&self) -> f64 {
        let left = self.left.unwrap_to_f64();
        let right = self.right.unwrap_to_f64();

        match self.operator.as_str() {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            // TODO: Switch to custom errors.
            _ => panic!("Invalid operator: {}.", &self.operator)
        }
    }

    fn execute(&self) -> ExpressionResult {
        match self.expression_type {
            ExpressionType::Unary => {
                ExpressionResult::Numeric(self.execute_unary())
            },
            ExpressionType::Binary => {
                ExpressionResult::Numeric(self.execute_binary())
            },
            ExpressionType::Numeric => {
                ExpressionResult::Numeric(self.execute_numeric())
            },
            _ => ExpressionResult::Void
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f, "({}: {}, {}, {})",
            &self.expression_type, &self.left, &self.operator, &self.right
        )
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        Expression {
            left: self.left.clone(),
            right: self.right.clone(),
            operator: self.operator.clone(),
            expression_type: self.expression_type.clone()
        }
    }
}
