use crate::backend::ast::{Expression, Statement};

use std::fmt::{Display, Formatter, Result};

pub enum StatementContainer {
    Void,
    Println {
        object: Expression
    },
    Assignment {
        identifier: String,
        object: Expression
    },
    IfElse {
        condition: Expression,
        if_statement: Statement,
        else_statement: Statement
    }
}

impl Display for StatementContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use StatementContainer::*;

        match self {
            Void => {
                write!(f, "void")
            },
            Println { object } => {
                write!(f, "println {object}")
            },
            Assignment { identifier, object } => {
                write!(f, "{identifier} = {object}")
            },
            IfElse { condition, if_statement, else_statement } => {
                write!(f, "if {condition} {{ {if_statement} }} else {{ {else_statement} }}")
            }
        }
    }
}
