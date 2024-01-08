use crate::backend::ast::{Expression, StatementContainer};
use crate::backend::parser::utils::IntermediateResult;
use crate::frontend::objects_driver::{NightObjectsDriver, NightValue};
use crate::frontend::utils::read_universal_input;
use crate::ok_r;

use std::fmt::{Display, Formatter, Result};

pub struct Statement {
    container: Box<StatementContainer>
}

impl Statement {
    pub fn void() -> Self {
        Self::new(StatementContainer::Void)
    }

    pub fn input(name: String) -> Self {
        Self::new(
            StatementContainer::Input { 
                name
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

    pub fn assignment(identifier: String, object: Expression) -> Self {
        Self::new(
            StatementContainer::Assignment {
                identifier,
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

    pub fn execute(&self, driver: &mut NightObjectsDriver) -> IntermediateResult {
        use StatementContainer::*;

        match self.container.as_ref() {
            Void => ok_r!("Void expression passed."),
            Input { name } => {
                let input = read_universal_input(">>> "); // Fix messages!
                let new_object = NightValue::Literal(input);

                driver.insert(name.to_owned(), new_object.clone());
                ok_r!("Successfully inserted `{name}` with value `{new_object}` into `NightObjectsDriver`.")
            },
            Println { object } => {
                let rvalue = object.execute(driver);
                println!("{}", &rvalue);

                ok_r!("Successfully printed {} to `stdout`.", rvalue)
            },
            Assignment { identifier, object } => {
                let rvalue = object.execute(driver);
                driver.insert(identifier.clone(), rvalue.clone());

                ok_r!("Successfully assigned value `{rvalue}` to object with name `{identifier}`.")
            },
            IfElse { condition, if_statement, else_statement } => {
                let condition_value = condition.execute(driver);
                let executing = if condition_value.unwrap_to_bool() { if_statement } else { else_statement };

                ok_r!("Because condition were `{}` executed this expression `{}` with result: {:?}", 
                    condition_value.unwrap_to_bool(), &executing, executing.execute(driver))
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
