use std::borrow::ToOwned;
use crate::backend::lexer::{Char, Position, is_lowercase, is_uppercase};
use crate::backend::tokens::{TokenType, Token};

pub const OPERATORS: [char; 12] = [
    '=', '(', ')', '+', '-', '*', '/', '<', '>', '!', '|', '&'
];

pub const RESERVED_KEYWORDS: [&str; 6] = [
    "if", "else", "elsif", "println", "true", "false",
];

pub const COMPLEX_OPERATORS_ENDINGS: [char; 3] = [
    '=', '|', '&'
];

pub const EOF_TOKEN: Token = Token::new(
    String::new(),
    TokenType::Eof,
    Position::new(usize::MAX, usize::MAX - 1)
);

pub fn define_identifier_type(buffer: &str) -> TokenType {
    if is_lowercase(buffer) {
        if RESERVED_KEYWORDS.contains(&buffer) {
            return match buffer {
                "if" => TokenType::IfKeyword,
                "else" => TokenType::ElseKeyword,
                "elsif" => TokenType::ElsIfKeyword,
                "println" => TokenType::Println,
                "true" => TokenType::BooleanValue(true),
                "false" => TokenType::BooleanValue(false),
                _ => TokenType::Broken(buffer.to_string())
            };
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
        '<' => TokenType::LessThan,
        '>' => TokenType::GreaterThan,
        '!' => TokenType::Inversion,
        '|' => TokenType::Or,
        '&' => TokenType::And,
        _ => TokenType::Broken(character.reference.to_string())
    }
}

pub fn define_complex_operator_type(operator: &str) -> TokenType {
    match operator {
        "<=" => TokenType::LessOrEqual,
        ">=" => TokenType::GreaterOrEqual,
        "==" => TokenType::Equal,
        "!=" => TokenType::NotEqual,
        "||" => TokenType::LogicalOr,
        "&&" => TokenType::LogicalAnd,
        _ => TokenType::Broken(operator.to_string())
    }
}

pub fn define_comment_type(character: &Char) -> TokenType {
    match character.reference {
        '@' => TokenType::Comment,
        '!' => TokenType::DocComment,
        _ => TokenType::Broken(character.reference.to_string())
    }
}
