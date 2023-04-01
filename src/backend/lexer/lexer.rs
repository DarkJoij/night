use crate::{spawn_night_error, if_debug};
use crate::backend::lexer::{
    Char,
    LexicalAssertions,
    LineManager
};
use crate::backend::tokens::{TokenType, Token};
use crate::backend::defaults::{
    define_operator_type, 
    define_identifier_type
};

pub struct Lexer {
    code: Vec<Char>,
    position: usize,
    tokens: Vec<Token>,
    line_manager: LineManager
}

impl Lexer {
    pub fn new(code_ref: &[u8]) -> Self {
        let code = code_ref.iter()
            .map(Char::new)
            .collect();

        Lexer {
            code, 
            position: 0, 
            tokens: Vec::new(),
            line_manager: LineManager::new(code_ref)
        }
    }

    pub fn lex(&mut self) -> &Vec<Token> {
        while self.position < self.code.len() {
            let current = self.peek();

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
                let error_line = self.line_manager.get_current_line(&self.position);
                spawn_night_error!(">>> {error_line}\nInvalid character found: {current:?}.");
            }
        }

        &self.tokens
    }

    fn add_token(&mut self, text: String, pd_type: TokenType) {
        self.tokens.push(Token::new(text, pd_type));
    }

    fn peek(&self) -> Char {
        match self.code.get(self.position) {
            Some(c) => *c,
            None => Char::new(&0)
        }
    }

    fn next(&mut self) -> Char {
        self.position += 1;
        self.peek()
    }

    fn lex_number(&mut self) {
        let mut buffer = String::new();
        let mut current = self.peek();

        loop {
            if current.equal('.') {
                if buffer.contains('.') {
                    spawn_night_error!(
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
            // Must be checked. It may not resolve infinity loop.
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
        let mut current = self.peek();
        let preliminary_type = define_identifier_type(&current);

        loop {
            buffer.push(current.reference);
            current = self.next();

            // This not covers all target types.
            // Must be refactored later.
            if !current.is_identifier() {
                break;
            }
        }

        self.add_token(buffer, preliminary_type)

        /*     
        match preliminary_type {
            TokenType::Broken(symbol) => {
                // 'symbol' might be formatted with "{:?}".
                spawn_night_error!("Invalid character ({symbol}) in position: {}.", &self.position);
            },
            _ => self.add_token(buffer, preliminary_type)
        }
        */
    }

    fn lex_operator(&mut self) {
        let current = self.peek();
        let preliminary_type = define_operator_type(&current);

        self.position += 1;
        self.add_token(current.to_string(), preliminary_type);

        /* 
        match preliminary_type {
            TokenType::Broken(operator) => {
                spawn_night_error!("Invalid operator ({operator}) at position {}.", &self.position);
            },
            _ => {
                self.position += 1;
                self.add_token(current.to_string(), preliminary_type);
            }
        }
        */
    }

    fn lex_comment(&mut self) {
        let mut buffer = String::new();
        let mut current = self.peek();

        loop {
            buffer.push(current.reference);
            current = self.next();

            if current.equal('\n') {
                break;
            }
        }

        if_debug! {
            self.add_token(buffer, TokenType::Comment);
        }
    }
}
