use crate::backend::ast::Expression;
use crate::backend::tokens::Token;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    expressions: Vec<Expression>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            expressions: Vec::new()
        }
    }

    pub fn parse(&self) -> &Vec<Expression> {
        &self.expressions
    }
}
