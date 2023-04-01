use crate::backend::tokens::TokenType;
use crate::backend::lexer::Char;

#[allow(dead_code)]
pub const IGNORE_CHARS: [char; 2] = [
    '\r', '\n'
];

pub const OPERATORS: [char; 5] = [
    '=', '+', '-', '*', '/'
];

#[allow(dead_code)]
pub const RESERVED_KEYWORDS: [String; 0] = [
    // Must be filled with 'print' and 'println' for 28.03.2023.
];

pub fn define_identifier_type(character: &Char) -> TokenType {
    if character.reference.is_uppercase() {
        return TokenType::Const
    }
    else if character.reference.is_lowercase() {
        return TokenType::Variable
    }
    
    TokenType::Broken(character.reference.to_string())
}

pub fn define_operator_type(character: &Char) -> TokenType {
    match character.reference {
        '=' => TokenType::Assignment, 
        '+' => TokenType::Addition, 
        '-' => TokenType::Subtraction, 
        '*' => TokenType::Multiplication, 
        '/' => TokenType::Division,
        _ => TokenType::Broken(character.reference.to_string())
    }
}
