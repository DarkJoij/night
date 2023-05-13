use crate::backend::ast::{Expression, StatementContainer};
use crate::frontend::objects_driver::NightObjectsDriver;

use std::fmt::{Display, Formatter, Result};

pub struct Statement {
    container: StatementContainer
}

impl Statement {
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

    pub fn execute(&self, driver: &mut NightObjectsDriver) {
        use StatementContainer::*;

        match &self.container {
            Void => (),
            Println { object } => {
                println!("{}", object.execute(driver));
            },
            Assignment { identifier, object } => {
                driver.insert(identifier.clone(), object.execute(driver));
            }
        }
    }

    fn new(container: StatementContainer) -> Self {
        Statement { container }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.container.fmt(f)
    }
}
