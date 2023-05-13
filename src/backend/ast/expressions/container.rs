use crate::backend::ast::Expression;

use std::fmt::{Display, Formatter, Result};

pub enum Container {
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

impl Display for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Container::*;

        match self {
            Atom { literal } => write!(f, "{literal}"),
            Unary { operator, operand } => write!(f, "{operator}{operand}"),
            Binary { operator, left, right } => write!(f, "{left} {operator} {right}")
        }
    }
}
