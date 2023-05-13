use std::borrow::ToOwned;
use crate::backend::lexer::{Char, Position, is_lowercase, is_uppercase};
use crate::backend::tokens::{TokenType, Token};

fn define_reserved_keyword_type(keyword: &str) -> TokenType {
    match keyword {
        "println" => TokenType::Println,
        _ => TokenType::Broken(keyword.to_string())
    }
}

pub const OPERATORS: [char; 7] = [
    '=', '(', ')', '+', '-', '*', '/'
];

pub const RESERVED_KEYWORDS: [&str; 1] = [
    "println"
];

pub const EOF_TOKEN: Token = Token::new(
    String::new(),
    TokenType::Eof,
    Position::new(usize::MAX, usize::MAX - 1)
);

pub fn define_identifier_type(buffer: &str) -> TokenType {
    if is_lowercase(buffer) {
        if RESERVED_KEYWORDS.contains(&buffer) {
            return define_reserved_keyword_type(buffer);
        }

        return TokenType::VariableIdentifier
    }
    else if is_uppercase(buffer) {
        return TokenType::ConstantIdentifier
    }
    
    TokenType::Broken(buffer.to_owned())
}

pub fn define_operator_type(character: &Char) -> TokenType {
    match character.reference {
        '=' => TokenType::Assignment,
        '(' => TokenType::LeftParenthesis,
        ')' => TokenType::RightParenthesis,
        '+' => TokenType::Addition, 
        '-' => TokenType::Subtraction, 
        '*' => TokenType::Multiplication, 
        '/' => TokenType::Division,
        _ => TokenType::Broken(character.reference.to_string())
    }
}

pub fn define_comment_type(character: &Char) -> TokenType {
    match character.reference {
        '@' => TokenType::Comment,
        '!' => TokenType::DocComment,
        _ => TokenType::Broken(character.reference.to_string())
    }
}
