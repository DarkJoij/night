use crate::{spawn_float_error, spawn_operator_error};
use crate::backend::ast::{
    DefaultExpression,
    ExpressionResult,
    ExpressionType,
    Value
};

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

// Actually later this will be fully rewritten.
impl DefaultExpression for Expression {
    fn execute_numeric(&self) -> f64 {
        let literal = self.left.unwrap_to_string();

        match literal.parse::<f64>() {
            Ok(n) => n,
            Err(_) => spawn_float_error!("Invalid numeric expression: {}.", &self.left)
        }
    }

    fn execute_unary(&self) -> f64 {
        let value = self.left.get_f64_from_expr();

        match self.operator.as_str() {
            "-" => -value,
            _ => value
        }
    }

    fn execute_binary(&self) -> f64 {
        let left = self.left.get_f64_from_expr();
        let right = self.right.get_f64_from_expr();

        match self.operator.as_str() {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => spawn_operator_error!("Invalid operator: {}.", &self.operator)
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
