use crate::backend::lexer::Position;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Data types:
    Number,
    String,
    // Identifiers:
    ConstantIdentifier,
    VariableIdentifier,
    // Operators:
    Assignment,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    // Control:
    LeftParenthesis,
    RightParenthesis,
    // User important:
    Comment,
    DocComment,
    // Core important:
    Eof,
    Broken(String)
}

// This should be checked later, as it
// may not be skipped by various linters.
impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenType::Broken(object) => write!(f, "Broken({object})"),
            _ => <Self as Debug>::fmt(self, f)
        }
    }
}

#[derive(Clone)]
pub struct Token {
    position: Position,

    pub text: String,
    pub pd_type: TokenType
}

impl Token {
    // `const` may be removed.
    pub const fn new(text: String, pd_type: TokenType, position: Position) -> Self {
        Token { text, pd_type, position }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.text)
    }
}

#[cfg(debug_assertions)]
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}: {:?} | {})", &self.pd_type, &self.text, &self.position)
    }
}
