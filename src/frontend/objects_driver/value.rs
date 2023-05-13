use crate::ub;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum NightValue {
    Void,
    Numeric(f64),
    Literal(String)
}

impl NightValue {
    pub fn unwrap_to_f64(&self) -> f64 {
        match self {
            // Might be replaced with `.clone()` calling.
            NightValue::Numeric(number) => *number,
            _ => ub!("Expected numeric expression instead of {self:?}.")
        }
    }

    pub fn unwrap_to_string(&self) -> String {
        match self {
            NightValue::Literal(string) => string.clone(),
            _ => ub!("Expected string expression instead of {self:?}.")
        }
    }
}

impl Display for NightValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use NightValue::*;

        match self {
            Void => write!(f, "void"),
            Numeric(number) => write!(f, "{number}"),
            Literal(string) => write!(f, "{string}")
        }
    }
}
