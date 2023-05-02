use crate::backend::lexer::Position;
use crate::backend::tokens::{TokenType, Token};

// Cause can't borrow constants.
pub static EOF_TOKEN: Token = Token::new(
    String::new(),
    TokenType::Eof,
    Position::new(usize::MAX, usize::MAX - 1)
);
