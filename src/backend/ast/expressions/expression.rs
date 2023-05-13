use crate::backend::ast::{
    Container,
    ExpressionType,
    ExpressionResult
};
use crate::{spawn_float_error, spawn_operator_error};

use std::fmt::{Display, Formatter, Result};

pub struct Expression {
    expression_type: ExpressionType,
    container: Box<Container>
}

impl Expression {
    pub fn new(expression_type: ExpressionType, container: Container) -> Self {
        Expression {
            expression_type,
            container: Box::new(container)
        }
    }

    pub fn numeric(literal: String) -> Self {
        Self::new(
            ExpressionType::Numeric,
            Container::Atom { literal }
        )
    }

    pub fn unary(operator: &str, operand: Expression) -> Self {
        Self::new(
            ExpressionType::Numeric,
            Container::Unary {
                operator: operator.to_owned(),
                operand
            }
        )
    }

    pub fn binary(operator: &str, left: Expression, right: Expression) -> Self {
        Self::new(
            ExpressionType::Numeric,
            Container::Binary {
                operator: operator.to_owned(),
                left,
                right
            }
        )
    }

    pub fn execute(&self) -> ExpressionResult {
        use ExpressionType::*;

        match self.expression_type {
            Void => ExpressionResult::Void,
            Numeric => ExpressionResult::Numeric(self.execute_numeric()),
            Literal => ExpressionResult::Literal(self.execute_literal())
        }
    }

    fn execute_numeric(&self) -> f64 {
        use Container::*;

        match self.container.as_ref() {
            Atom { literal } => {
                match literal.parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => spawn_float_error!("Invalid numeric expression: {}.", &literal)
                }
            },
            Unary { operator, operand } => {
                let value = operand.execute()
                    .unwrap_to_f64();

                match operator.as_str() {
                    "-" => -value,
                    _ => value
                }
            },
            Binary { operator, left, right } => {
                let left_value = left.execute()
                    .unwrap_to_f64();
                let right_value = right.execute()
                    .unwrap_to_f64();

                match operator.as_str() {
                    "+" => left_value + right_value,
                    "-" => left_value - right_value,
                    "*" => left_value * right_value,
                    "/" => left_value / right_value,
                    _ => spawn_operator_error!("Invalid operator: {}.", &operator)
                }
            }
        }
    }

    fn execute_literal(&self) -> String {
        String::new()
    }
}

/// Димка ты прав, тут лучше [`Display`].
impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.container)
    }
}
