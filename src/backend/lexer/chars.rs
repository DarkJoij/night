use crate::backend::defaults::OPERATORS;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Copy)]
pub struct Char {
    pub reference: char
}

pub trait LexicalAssertions {
    fn is_numeric(&self) -> bool;
    fn is_quote(&self) -> bool;
    fn is_operator(&self) -> bool;
    fn is_doc_comment(&self) -> bool;
    fn is_comment(&self) -> bool;
    fn is_reserved_keyword(&self) -> bool;
    fn is_identifier(&self) -> bool;
    fn is_whitespace(&self) -> bool;
    fn is_eof(&self) -> bool;
    fn equal(&self, other: char) -> bool;
}

impl Char {
    pub fn new(char_code: &u8) -> Self {
        Char { 
            reference: (*char_code) as char
        }
    }
}

impl LexicalAssertions for Char {
    fn is_numeric(&self) -> bool {
        self.reference.is_numeric()
    }

    fn is_quote(&self) -> bool {
        self.reference == '"'
    }

    fn is_operator(&self) -> bool {
        OPERATORS.contains(&self.reference)
    }

    fn is_doc_comment(&self) -> bool {
        self.reference == '!'
    }

    fn is_comment(&self) -> bool {
        self.reference == '@'
    }

    // Must be implemented later.
    fn is_reserved_keyword(&self) -> bool {
        todo!()
    }

    fn is_identifier(&self) -> bool {
        self.reference == '_'
            || self.reference.is_uppercase()
            || self.reference.is_lowercase()
    }

    fn is_whitespace(&self) -> bool {
        self.reference == ' '
            || self.reference == '\r'
            || self.reference == '\n'
    }

    fn is_eof(&self) -> bool {
        self.reference == '\0'
    }

    fn equal(&self, other: char) -> bool {
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
