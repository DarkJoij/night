use crate::backend::ast::{Expression, StatementContainer};
use crate::frontend::objects_driver::NightObjectsDriver;

use std::fmt::{Display, Formatter, Result};

pub struct Statement {
    container: Box<StatementContainer>
}

impl Statement {
    pub fn void() -> Self {
        Self::new(StatementContainer::Void)
    }

    pub fn assignment(identifier: String, object: Expression) -> Self {
        Self::new(
            StatementContainer::Assignment {
                identifier,
                object
            }
        )
    }

    pub fn println(object: Expression) -> Self {
        Self::new(
            StatementContainer::Println {
                object
            }
        )
    }

    pub fn if_else(
        condition: Expression,
        if_statement: Statement,
        else_statement: Statement
    ) -> Self {
        Self::new(
            StatementContainer::IfElse {
                condition,
                if_statement,
                else_statement
            }
        )
    }

    pub fn execute(&self, driver: &mut NightObjectsDriver) {
        use StatementContainer::*;

        match self.container.as_ref() {
            Void => (),
            Println { object } => {
                println!("{}", object.execute(driver));
            },
            Assignment { identifier, object } => {
                driver.insert(identifier.clone(), object.execute(driver));
            },
            IfElse { condition, if_statement, else_statement } => {
                let condition_value = condition.execute(driver);

                if condition_value.unwrap_to_bool() {
                    if_statement.execute(driver);
                }
                else {
                    else_statement.execute(driver);
                }
            }
        }
    }

    fn new(container: StatementContainer) -> Self {
        Statement {
            container: Box::new(container)
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.container.fmt(f)
    }
}
