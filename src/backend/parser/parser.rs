use crate::backend::ast::{Expression, Statement};
use crate::backend::defaults::EOF_TOKEN;
use crate::backend::lexer::LineManager;
use crate::backend::tokens::{TokenType, Token};
use crate::spawn_syntax_error;

pub struct Parser<'a> {
    position: usize,
    tokens: &'a Vec<Token>,
    line_manager: &'a LineManager
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>, line_manager: &'a LineManager) -> Self {
        Parser {
            position: 0,
            tokens,
            line_manager
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut expressions = Vec::new();

        while !self.match_type(TokenType::Eof) {
            expressions.push(self.statement())
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

    fn consume(&mut self, expected_type: TokenType) -> bool {
        let compared = self.match_type(expected_type.clone());

        if !compared {
            spawn_syntax_error!(
                "Expected {expected_type} expression instead of '{}'.", self.get_token(0)
            );
        }

        compared
    }

    fn statement(&mut self) -> Statement {
        if self.match_type(TokenType::Println) {
            return Statement::println(self.expression());
        }

        self.assignment_statement()
    }

    fn assignment_statement(&mut self) -> Statement {
        let current_token = self.get_token(0);

        if self.consume(TokenType::VariableIdentifier)
            && self.consume(TokenType::Assignment)
        {
            return Statement::assignment(current_token.text, self.expression());
        }

        spawn_syntax_error!("Invalid expression: {current_token}")
    }

    fn expression(&mut self) -> Expression {
        self.additive()
    }

    fn additive(&mut self) -> Expression {
        let mut expression = self.multiplicative();

        loop {
            if self.match_type(TokenType::Addition) {
                expression = Expression::binary("+", expression, self.multiplicative());
                continue;
            }
            if self.match_type(TokenType::Subtraction) {
                expression = Expression::binary("-", expression, self.multiplicative());
                continue;
            }

            break;
        }

        expression
    }

    fn multiplicative(&mut self) -> Expression {
        let mut expression = self.unary();

        loop {
            if self.match_type(TokenType::Multiplication) {
                expression = Expression::binary("*", expression, self.unary());
                continue;
            }
            if self.match_type(TokenType::Division) {
                expression = Expression::binary("/", expression, self.unary());
                continue;
            }

            break;
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if self.match_type(TokenType::Subtraction) {
            return Expression::unary("-", self.primary())
        }

        self.match_type(TokenType::Addition);
        self.primary()
    }

    fn primary(&mut self) -> Expression {
        let current_token = self.get_token(0);

        if self.match_type(TokenType::String) {
            return Expression::literal(current_token.text);
        }
        if self.match_type(TokenType::Number) {
            return Expression::numeric(current_token.text);
        }
        if self.match_type(TokenType::ConstantIdentifier) {
            return Expression::constant(current_token.text);
        }
        if self.match_type(TokenType::VariableIdentifier) {
            return Expression::variable(current_token.text);
        }

        if self.match_type(TokenType::LeftParenthesis) {
            let expression = self.expression();
            self.match_type(TokenType::RightParenthesis);

            return expression;
        }

        spawn_syntax_error!("Invalid syntax: {current_token:?}.")
    }
}
