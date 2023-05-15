use crate::backend::ast::Expression;
use crate::ub;

use std::fmt::{Display, Formatter, Result};

pub enum ExpressionContainer {
    Atom {
        literal: String
    },
    Unary {
        operator: String,
        operand: Expression
    },
    Binary {
        operator: String,
        left: Expression,
        right: Expression
    }
}

impl ExpressionContainer {
    pub fn unwrap_atom(&self) -> &String {
        match self {
            ExpressionContainer::Atom { literal } => literal,
            _ => ub!("Expected identifier instead of '{self}'.")
        }
    }
}

impl Display for ExpressionContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ExpressionContainer::*;

        match self {
            Atom { literal } => write!(f, "{literal}"),
            Unary { operator, operand } => write!(f, "{operator}{operand}"),
            Binary { operator, left, right } => write!(f, "{left} {operator} {right}")
        }
    }
}
