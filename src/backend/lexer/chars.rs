use crate::backend::defaults::OPERATORS;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Copy)]
pub struct Char {
    pub reference: char
}

impl Char {
    pub fn new(char_code: &u8) -> Self {
        Char { 
            reference: (*char_code) as char
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.reference.is_numeric()
    }

    pub fn is_quote(&self) -> bool {
        self.reference == '"'
    }

    pub fn is_operator(&self) -> bool {
        OPERATORS.contains(&self.reference)
    }

    pub fn is_doc_comment(&self) -> bool {
        self.reference == '!'
    }

    pub fn is_comment(&self) -> bool {
        self.reference == '@'
    }

    // Must be implemented later.
    pub fn is_reserved_keyword(&self) -> bool {
        todo!()
    }

    pub fn is_identifier(&self) -> bool {
        self.reference == '_'
            || self.reference.is_numeric()
            || self.reference.is_uppercase()
            || self.reference.is_lowercase()
    }

    pub fn is_whitespace(&self) -> bool {
        self.reference == ' '
            || self.reference == '\r'
            || self.reference == '\n'
    }

    pub fn is_eof(&self) -> bool {
        self.reference == '\0'
    }

    pub fn equal(&self, other: char) -> bool {
        self.reference == other
    }
}

#[cfg(debug_assertions)]
impl Debug for Char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", &self.reference)
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.reference)
    }
}

pub fn is_lowercase(buffer: &str) -> bool {
    for character in buffer.chars() {
        if !character.is_lowercase() && !character.is_numeric() && character != '_'  {
            return false;
        }
    }

    true
}

pub fn is_uppercase(buffer: &str) -> bool {
    for character in buffer.chars() {
        if !character.is_uppercase() && !character.is_numeric() && character != '_' {
            return false;
        }
    }

    true
}
