use crate::backend::ast::Expression;

use std::fmt::{Display, Formatter, Result};

pub enum StatementContainer {
    Void, // Must be remove later.
    Println {
        object: Expression
    },
    Assignment {
        identifier: String,
        object: Expression
    }
}

impl Display for StatementContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use StatementContainer::*;

        match self {
            Void => write!(f, "()"),
            Println { object } => write!(f, "println {object}"),
            Assignment { identifier, object } => write!(f, "{identifier} = {object}")
        }
    }
}
