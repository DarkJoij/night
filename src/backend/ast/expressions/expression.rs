use crate::backend::ast::{ExpressionContainer, ExpressionExecutor};
use crate::frontend::objects_driver::{NightObjectsDriver, NightValue};

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ExpressionType {
    Numeric,
    Literal,
    Constant,
    Variable
}

pub struct Expression {
    expression_type: ExpressionType,
    container: Box<ExpressionContainer>
}

impl Expression {
    pub fn numeric(literal: String) -> Self {
        Self::new(
            ExpressionType::Numeric,
            ExpressionContainer::Atom { literal }
        )
    }

    pub fn literal(literal: String) -> Self {
        Self::new(
            ExpressionType::Literal,
            ExpressionContainer::Atom { literal }
        )
    }

    pub fn unary(operator: &str, operand: Expression) -> Self {
        Self::new(
            ExpressionType::Numeric,
            ExpressionContainer::Unary {
                operator: operator.to_string(),
                operand
            }
        )
    }

    pub fn binary(operator: &str, left: Expression, right: Expression) -> Self {
        Self::new(
            ExpressionType::Numeric,
            ExpressionContainer::Binary {
                operator: operator.to_owned(),
                left,
                right
            }
        )
    }

    pub fn constant(identifier: String) -> Self {
        Self::new(
            ExpressionType::Constant,
            ExpressionContainer::Atom { literal: identifier }
        )
    }

    pub fn variable(identifier: String) -> Self {
        Self::new(
            ExpressionType::Variable,
            ExpressionContainer::Atom { literal: identifier }
        )
    }

    pub fn execute(&self, driver: &NightObjectsDriver) -> NightValue {
        let executor = ExpressionExecutor::new(
            driver,
            self.container.as_ref(),
            &self.expression_type
        );

        executor.execute()
    }

    fn new(expression_type: ExpressionType, container: ExpressionContainer) -> Self {
        Expression {
            expression_type,
            container: Box::new(container)
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.container.fmt(f)
    }
}
