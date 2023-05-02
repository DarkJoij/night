use crate::backend::ast::Expression;
use crate::backend::parser::EOF_TOKEN; // Must path be specified?
use crate::backend::tokens::{TokenType, Token};
use crate::spawn_core_error;

pub struct Parser<'a> {
    position: usize,
    tokens: &'a Vec<Token>,
    expressions: Vec<Expression>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            position: 0,
            tokens,
            expressions: Vec::new()
        }
    }

    pub fn parse(&self) -> &Vec<Expression> {
        &self.expressions
    }

    fn get_token(&self, forward_on: usize) -> &Token {
        match self.tokens.get(self.position + forward_on) {
            None => &EOF_TOKEN,
            Some(t) => t
        }
    }

    fn match_type(&mut self, expected_type: TokenType) -> bool {
        let current_token = self.get_token(0);

        if current_token.pd_type != expected_type {
            return false;
        }

        self.position += 1;
        true
    }

    fn expression(&mut self) -> Expression {
        self.multiplicative()
    }

    fn additive(&mut self) -> Expression {
        self.unary()
    }

    fn multiplicative(&mut self) -> Expression {
        self.unary()
    }

    fn unary(&mut self) -> Expression {
        self.primary()
    }

    fn primary(&mut self) -> Expression {
        let current_token = self.get_token(0);

        if self.match_type(TokenType::Number) {
            return Expression::numeric(current_token.text);
        }

        spawn_core_error!("Failed to pass `primary` function.")
    }
}
