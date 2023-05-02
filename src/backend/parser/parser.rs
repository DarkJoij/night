use crate::backend::ast::Expression;
use crate::backend::defaults::EOF_TOKEN;
use crate::backend::lexer::LineManager;
use crate::backend::tokens::{TokenType, Token};
use crate::spawn_core_error;

pub struct Parser<'a> {
    position: usize,
    tokens: &'a Vec<Token>,
    expressions: Vec<Expression>,
    line_manager: &'a LineManager
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, line_manager: &'a LineManager) -> Self {
        Parser {
            position: 0,
            tokens,
            expressions: Vec::new(),
            line_manager
        }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();

        while !self.match_type(TokenType::Eof) {
            expressions.push(self.expression())
        }

        expressions
    }

    fn get_token(&self, forward_on: usize) -> Token {
        match self.tokens.get(self.position + forward_on) {
            None => EOF_TOKEN,
            Some(t) => t.clone()
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
        self.additive()
    }

    fn additive(&mut self) -> Expression {
        let mut expression = self.multiplicative();

        loop {
            // `expression` and `self.multiplicative()` may be replaced. Need to check.
            if self.match_type(TokenType::Addition) {
                expression = Expression::binary(expression, self.multiplicative(), "+");
                continue;
            }
            if self.match_type(TokenType::Subtraction) {
                expression = Expression::binary(expression, self.multiplicative(), "-");
                continue;
            }

            break;
        }

        expression
    }

    fn multiplicative(&mut self) -> Expression {
        let mut expression = self.unary();

        loop {
            // `expression` and `self.unary()` may be replaced. Need to check.
            if self.match_type(TokenType::Multiplication) {
                expression = Expression::binary(expression, self.unary(), "*");
                continue;
            }
            if self.match_type(TokenType::Division) {
                expression = Expression::binary(expression, self.unary(), "/");
                continue;
            }

            break;
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        // if self.match_type(TokenType::Subtraction) {
        //
        // }

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
