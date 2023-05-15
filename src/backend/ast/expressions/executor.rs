use crate::backend::ast::{ExpressionContainer, ExpressionType};
use crate::frontend::objects_driver::{NightObjectsDriver, NightValue};
use crate::{
    spawn_float_error,
    spawn_operator_error,
    spawn_syntax_error,
    spawn_type_error,
    ub
};

use std::mem::discriminant;

fn arms_equals<T>(left: &T, right: &T) -> bool {
    discriminant(left) == discriminant(right)
}

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
            Boolean => self.execute_boolean(),
            Constant => self.execute_constant(),
            Variable => self.execute_variable()
        }
    }

    fn execute_numeric(&self) -> NightValue {
        use ExpressionContainer::*;

        let calculated = match self.container {
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
            Atom { literal } => literal.clone(),
            Binary { operator, left, right } => {
                let right_value = right.execute(self.driver);
                let mut left_string = left.execute(self.driver)
                    .unwrap_to_string();

                match operator.as_str() {
                    "+" => {
                        let right_string = right_value.unwrap_to_string();
                        left_string.push_str(&right_string);

                        left_string
                    },
                    "*" => {
                        let times = right_value.unwrap_to_f64();
                        left_string.repeat(times as usize)
                    },
                    _ => left_string
                }
            },
            _ => spawn_syntax_error!(
                "Invalid operation applied to string: {}", &self.container
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

    fn execute_boolean(&self) -> NightValue {
        use ExpressionContainer::*;

        fn failed_to_apply(operator: &String, objects: &str) -> ! {
            spawn_type_error!(
                "Failed to apply ({}) operation between {}.", operator, objects
            )
        }

        let boolean = match self.container {
            Unary { operator, operand } => {
                let value = operand.execute(self.driver);

                match operator.as_str() {
                    "!" => !value.unwrap_to_bool(),
                    _ => spawn_operator_error!("Invalid boolean operator: {operator}.")
                }
            },
            Binary { operator, left, right } => {
                use NightValue::*;

                let left_value = left.execute(self.driver);
                let right_value = right.execute(self.driver);

                if !arms_equals(&left_value, &right_value) {
                    failed_to_apply(operator, "different types");
                }

                match left_value {
                    Void => false,
                    Numeric(left_number) => {
                        match operator.as_str() {
                            "<" => left_number < right_value.unwrap_to_f64(),
                            ">" => left_number > right_value.unwrap_to_f64(),
                            "<=" => left_number <= right_value.unwrap_to_f64(),
                            ">=" => left_number >= right_value.unwrap_to_f64(),
                            "==" => left_number == right_value.unwrap_to_f64(),
                            "!=" => left_number != right_value.unwrap_to_f64(),
                            _ => failed_to_apply(operator, "numbers")
                        }
                    },
                    Literal(left_string) => {
                        match operator.as_str() {
                            // "<", ">", "<=" and ">=" must be covered.
                            "==" => left_string == right_value.unwrap_to_string(),
                            "!=" => left_string != right_value.unwrap_to_string(),
                            _ => failed_to_apply(operator, "strings")
                        }
                    },
                    Boolean(left_boolean) => {
                        match operator.as_str() {
                            "|" => left_boolean | right_value.unwrap_to_bool(),
                            "&" => left_boolean & right_value.unwrap_to_bool(),
                            "==" => left_boolean == right_value.unwrap_to_bool(),
                            "!=" => left_boolean != right_value.unwrap_to_bool(),
                            "||" => left_boolean || right_value.unwrap_to_bool(),
                            "&&" => left_boolean && right_value.unwrap_to_bool(),
                            _ => failed_to_apply(operator, "booleans")
                        }
                    }
                }
            },
            _ => ub!("Invalid operation applied to boolean: {}.", self.container)
        };

        NightValue::Boolean(boolean)
    }
}
