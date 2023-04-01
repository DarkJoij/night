use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum TokenType {
    // Data types:
    Number,
    String,
    // Identifiers:
    Const,
    Variable,
    // Operators:
    Assignment,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    // User important:
    Comment,
    // Core important:
    Broken(String)
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenType::Broken(object) => write!(f, "{}", object),
            _ => <Self as Debug>::fmt(&self, f)
        }
    }
}

pub struct Token {
    text: String,
    pd_type: TokenType
}

impl Token {
    pub fn new(text: String, pd_type: TokenType) -> Self {
        Token { text, pd_type }
    }
}

#[cfg(debug_assertions)]
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<{:?}, {}>", &self.text, &self.pd_type)
    }
}
