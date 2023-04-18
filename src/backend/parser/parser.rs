use crate::backend::ast::Expression;
use crate::backend::tokens::Token;

use std::fmt::Debug;

pub struct Parser<'a, T, O>
where
    T: Debug,
    O: Debug + Default
{
    tokens: &'a Vec<Token>,
    expressions: Vec<Expression<T, O>>
}

impl<'a, T, O> Parser<'a, T, O>
where
    T: Debug,
    O: Debug + Default
{
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            expressions: Vec::new()
        }
    }

    pub fn parse(&self) -> &Vec<Expression<T, O>> {
        &self.expressions
    }
}
