use crate::backend::ast::{ExpressionContainer, ExpressionType};
use crate::frontend::objects_driver::{NightObjectsDriver, NightValue};
use crate::{if_debug, spawn_float_error, spawn_operator_error, spawn_syntax_error};

pub struct ExpressionExecutor<'a> {
    driver: &'a NightObjectsDriver,
    container: &'a ExpressionContainer,
    expression_type: &'a ExpressionType
}

impl<'a> ExpressionExecutor<'a> {
    pub fn new(
        driver: &'a NightObjectsDriver,
        container: &'a ExpressionContainer,
        expression_type: &'a ExpressionType
    ) -> Self {
        ExpressionExecutor {
            container,
            driver,
            expression_type
        }
    }

    pub fn execute(&self) -> NightValue {
        use ExpressionType::*;

        match self.expression_type {
            Numeric => self.execute_numeric(),
            Literal => self.execute_literal(),
            Constant => self.execute_constant(),
            Variable => self.execute_variable()
        }
    }

    fn execute_numeric(&self) -> NightValue {
        use ExpressionContainer::*;

        let calculated = match self.container {
            Void => f64::NAN,
            Atom { literal } => {
                match literal.parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => spawn_float_error!("Invalid numeric expression: {}.", &literal)
                }
            },
            Unary { operator, operand } => {
                let value = operand.execute(self.driver)
                    .unwrap_to_f64();

                match operator.as_str() {
                    "-" => -value,
                    _ => value
                }
            },
            Binary { operator, left, right } => {
                let left_value = left.execute(self.driver)
                    .unwrap_to_f64();
                let right_value = right.execute(self.driver)
                    .unwrap_to_f64();

                match operator.as_str() {
                    "+" => left_value + right_value,
                    "-" => left_value - right_value,
                    "*" => left_value * right_value,
                    "/" => left_value / right_value,
                    _ => spawn_operator_error!("Invalid operator: {}.", &operator)
                }
            }
        };

        NightValue::Numeric(calculated)
    }

    fn execute_literal(&self) -> NightValue {
        use ExpressionContainer::*;

        let string = match self.container {
            Void => String::new(),
            Atom { literal } => literal.clone(),
            _ => spawn_syntax_error!(
                "Invalid operation on strings: {}", &self.container
            )
        };

        NightValue::Literal(string)
    }

    fn execute_constant(&self) -> NightValue {
        self.driver.get(self.container.unwrap_atom())
    }

    fn execute_variable(&self) -> NightValue {
        self.execute_constant()
    }
}
