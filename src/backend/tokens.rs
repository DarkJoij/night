use std::fmt::{Debug, Display, Formatter, Result};

use crate::backend::lexer::Position;

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
    text: String,
    pd_type: TokenType,
    // position: Position
}

impl Token {
    pub fn new(
        text: String, pd_type: TokenType,
        // line: usize, column: usize
    ) -> Self {
        Token { text, pd_type }
    }
}

#[cfg(debug_assertions)]
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<{:?}, {}>", &self.text, &self.pd_type)
    }
}
