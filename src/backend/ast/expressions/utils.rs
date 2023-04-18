use crate::{spawn_core_error, spawn_type_error};
use crate::backend::ast::{
    Expression,
    ExpressionCallback,
    ExpressionType
};

use std::fmt::Debug;

pub fn assert_recursive<T, O>(expression: &Expression<T, O>)
    where
        T: Debug,
        O: Debug + Default
{
    if expression.expression_type != ExpressionType::Recursive {
        spawn_type_error!(
            "Expression `{{{expression:?}}}` has invalid type: {:?}.",
            expression.expression_type
        )
    }
}

pub fn unwrap_to_f64(callback: ExpressionCallback) -> f64 {
    match callback {
        ExpressionCallback::Numeric(number) => number,
        _ => spawn_core_error!(
            "Unexpected non-numeric expressions met: {callback:?}."
        )
    }
}
