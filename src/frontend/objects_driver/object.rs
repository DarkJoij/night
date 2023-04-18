//! In my opinion, it should be refactored,
//! since the quality of the code is low.

use crate::spawn_float_error;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum NightObjectType {
    String,
    Number,
    Struct(String)
}

// This should be checked later, as it
// may not be skipped by various linters.
impl Display for NightObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            NightObjectType::Struct(literal) => write!(f, "{literal}"),
            _ => <Self as Debug>::fmt(self, f)
        }
    }
}

pub struct NightObject {
    text: String,
    final_type: NightObjectType,

    pub identifier: String
}

pub trait TypeInference {
    fn as_number(&self) -> f64;
    fn as_string(&self) -> String;
    fn as_struct(&self) -> NightObjectType;
}

impl NightObject {
    pub fn new(
        text: String, identifier: String, final_type: NightObjectType
    ) -> Self {
        NightObject { text, identifier, final_type }
    }

    pub fn as_string(&self) -> String {
        self.text.clone()
    }

    pub fn as_number(&self) -> f64 {
        match self.text.parse::<f64>() {
            Ok(n) => n,
            Err(n) => spawn_float_error!("Invalid number provided: {n}.")
        }
    }

    // Must be implemented later.
    pub fn as_custom_type(&self) -> String {
        todo!()
    }
}

#[cfg(debug_assertions)]
impl Debug for NightObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: {} = {}", &self.identifier, &self.final_type, &self.text)
    }
}
