use crate::{spawn_core_error, spawn_operator_error};
use crate::backend::ast::{
    BinaryExpression,
    DefaultExpression,
    ExpressionCallback,
    ExpressionType,
    NumericExpression,
    UnaryExpression,

    // assert_recursive,
    unwrap_to_f64
};

use std::fmt::Debug;

/// Should be refactored, since it
/// is a wrapper for [`Expression`] fields.
pub struct Container<V, O> {
    /* pub */ values: Vec<V>,
    /* pub */ supportive: O
}

impl<V, O: Default> Container<V, O> {
    pub fn single(value: V) -> Self {
        Container {
            values: vec![value],
            supportive: O::default()
        }
    }

    pub fn double(left: V, right: V) -> Self {
        Container {
            values: vec![left, right],
            supportive: O::default()
        }
    }

    pub fn triple(left: V, center: O, right: V) -> Self {
        Container {
            values: vec![left, right],
            supportive: center
        }
    }
}

#[derive(Debug)] // TODO: Remove me with `Display`!
pub struct Expression<T, O>
where
    T: Debug,
    O: Debug + Default
{
    container: Container<T, O>,

    pub expression_type: ExpressionType
}

impl<T, O> Expression<T, O>
where
    T: Debug,
    O: Debug + Default
{
    pub fn numeric(value: T) -> Self {
        Expression {
            container: Container::single(value),
            expression_type: ExpressionType::Numeric
        }
    }

    pub fn binary(left: T, operator: O, right: T) -> Self {
        Expression {
            container: Container::triple(left, operator, right),
            expression_type: ExpressionType::Binary
        }
    }

    pub fn get_from_container(&self, index: usize) -> &T {
        match self.container.values.get(index) {
            Some(v) => v,
            None => spawn_core_error!("Failed to get value from buffer at: {index}")
        }
    }
}

impl<O> NumericExpression<f64> for Expression<f64, O>
where
    O: Debug + Default
{
    fn execute_numeric(&self) -> f64 {
        *self.get_from_container(0) // TODO: Must be refactored. `Deref` is not the best solution.
    }
}

impl<T, O> BinaryExpression<f64> for Expression<Expression<T, O>, String>
where
    T: Debug,
    O: Debug + Default
{
    fn execute_binary(&self) -> f64 {
        let left = self.get_from_container(0);
        let right = self.get_from_container(1);

        // assert_recursive(&left); // FIXME: This may kill interpreter.
        // assert_recursive(&right);

        let calculated_left = unwrap_to_f64(left.execute());
        let calculated_right = unwrap_to_f64(right.execute());

        match self.container.supportive.as_str() {
            "+" => calculated_left + calculated_right,
            "-" => calculated_left - calculated_right,
            "*" => calculated_left * calculated_right,
            "/" => calculated_left / calculated_right,
            _ => spawn_operator_error!("Invalid operator: '{}'.", self.container.supportive)
        }
    }
}

impl<T, O> UnaryExpression<f64> for Expression<Expression<T, O>, String>
where
    T: Debug,
    O: Debug + Default
{
    fn execute_unary(&self) -> f64 {
        let operand = self.get_from_container(0);
        let operand_value = unwrap_to_f64(operand.execute());

        match self.container.supportive.as_str() {
            "-" => -operand_value,
            _ => spawn_operator_error!("Invalid operator: '{}'.", self.container.supportive)
        }
    }
}

impl<T, O> DefaultExpression for Expression<T, O>
where
    T: Debug,
    O: Debug + Default
{
    fn execute(&self) -> ExpressionCallback {
        match self.expression_type {
            ExpressionType::Numeric => ExpressionCallback::Numeric(
                <Self as NumericExpression<f64>>::execute_numeric(self)
            ),
            ExpressionType::Binary => ExpressionCallback::Numeric(
                <Self as BinaryExpression<f64>>::execute_binary(self)
            ),
            ExpressionType::Unary => {}
            _ => {}
        }
    }
}
