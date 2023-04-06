use crate::backend::tokens::Token;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&self) -> &Token {
        let x = self.tokens.get(1);
        x.unwrap()
    }
}
