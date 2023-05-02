use crate::backend::lexer::Position;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, PartialEq)]
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
            TokenType::Broken(object) => write!(f, "{object}"),
            _ => <Self as Debug>::fmt(self, f)
        }
    }
}

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

#[cfg(debug_assertions)]
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<{:?}, {} ~{}>", &self.text, &self.pd_type, &self.position)
    }
}
