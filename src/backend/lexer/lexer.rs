use crate::backend::lexer::{Char, LineManager};
use crate::backend::tokens::{TokenType, Token};
use crate::backend::defaults::{
    COMPLEX_OPERATORS_ENDINGS,
    define_operator_type,
    define_complex_operator_type,
    define_identifier_type,
    define_comment_type
};
use crate::{if_daily, spawn_syntax_error, spawn_float_error};

pub struct Lexer<'a> {
    code: Vec<Char>,
    position: usize,
    tokens: Vec<Token>,
    line_manager: &'a LineManager
}

impl<'a> Lexer<'a> {
    pub fn new(code_ref: &[u8], line_manager: &'a LineManager) -> Self {
        let code = code_ref.iter()
            .map(Char::new)
            .collect();

        Lexer {
            code,
            position: 0, 
            tokens: Vec::new(),
            line_manager
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.position < self.code.len() {
            let current = self.peek(0);

            if current.is_numeric() {
                self.lex_number();
            }
            else if current.is_quote() {
                self.lex_string();
            }
            else if current.is_identifier() {
                self.lex_identifier();
            }
            else if current.is_operator() {
                self.lex_operator();
            }
            else if current.is_comment() {
                self.lex_comment();
            }
            else if current.is_whitespace() {
                self.position += 1;
            }
            else {
                let cursor = self.line_manager.get_cursor(self.position);
                spawn_syntax_error!("\n{cursor}\nInvalid character found: {current:?}.");
            }
        }

        self.tokens.clone()
    }

    fn add_token(&mut self, text: String, pd_type: TokenType) {
        let mut length = text.len();

        if pd_type == TokenType::String {
            length += 2;
        }

        let position = self.line_manager.get_position(self.position - length);
        self.tokens.push(Token::new(text, pd_type, position));
    }

    fn peek(&self, forward_on: usize) -> Char {
        match self.code.get(self.position + forward_on) {
            None => Char::new(&0),
            Some(c) => *c
        }
    }

    fn next(&mut self) -> Char {
        self.position += 1;
        self.peek(0)
    }

    fn lex_number(&mut self) {
        let mut buffer = String::new();
        let mut current = self.peek(0);

        loop {
            if current.equal('.') {
                if buffer.contains('.') {
                    spawn_float_error!(
                        "Invalid floating point number at position: {}.", &self.position
                    );
                }
            } 
            else if !current.is_numeric() {
                break;
            }

            buffer.push(current.reference);
            current = self.next();
        }

        self.add_token(buffer, TokenType::Number);
    }

    fn lex_string(&mut self) {
        let mut buffer = String::new();
        let mut current = self.next();

        loop {
            if current.equal('\\') {
                current = self.next();
                buffer.push(current.reference);

                current = self.next();
                continue;
            }
            if current.equal('"') || current.is_eof() {
                break;
            }

            buffer.push(current.reference);
            current = self.next();
        }

        self.position += 1;
        self.add_token(buffer, TokenType::String);
    }

    fn lex_identifier(&mut self) {
        let mut buffer = String::new();
        let mut current = self.peek(0);

        loop {
            buffer.push(current.reference);
            current = self.next();

            if !current.is_identifier() {
                break;
            }
        }

        let preliminary_type = define_identifier_type(&buffer);
        self.add_token(buffer, preliminary_type);
    }

    fn lex_operator(&mut self) {
        let current = self.peek(0);
        let next = self.peek(1);

        let mut buffer = String::from(current.reference);

        let preliminary_type = if COMPLEX_OPERATORS_ENDINGS.contains(&next.reference) {
            buffer.push(next.reference);
            self.position += 1;

            define_complex_operator_type(&buffer)
        } else {
            define_operator_type(&current)
        };

        self.position += 1;
        self.add_token(buffer, preliminary_type);
    }

    fn lex_comment(&mut self) {
        let mut buffer = String::new();
        let mut current = self.peek(0);
        let next = self.peek(1);

        if !next.is_comment() && !next.is_doc_comment() {
            let cursor = self.line_manager.get_cursor(self.position);

            spawn_syntax_error!(
                "\n{cursor}\nInvalid comment declaration at {}.",
                cursor.position
            );
        }

        loop {
            buffer.push(current.reference);
            current = self.next();

            if current.equal('\n') {
                break;
            }
        }

        if_daily! {
            self.add_token(buffer, define_comment_type(&next));
        }
    }
}
