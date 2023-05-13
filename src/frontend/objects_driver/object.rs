use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum NightValue {
    Void,
    Numeric(f64),
    Literal(String)
}

impl Display for NightValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use NightValue::*;

        match self {
            Void => write!(f, "void"),
            Numeric(number) => write!(f, "{number}"),
            Literal(string) => write!(f, "{string}")
        }
    }
}

pub struct NightObject {
    name: String,
    value: NightValue
}

impl NightObject {
    pub fn new(name: String, value: NightValue) -> Self {
        NightObject { name, value }
    }
}
