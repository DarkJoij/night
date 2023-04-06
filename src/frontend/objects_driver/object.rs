use crate::spawn_float_error;

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum NightObjectType {
    String,
    Number,
    Custom(String)
}

impl Display for NightObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            NightObjectType::Custom(literal) => write!(f, "{literal}"),
            _ => <Self as Debug>::fmt(self, f)
        }
    }
}

pub struct NightObject {
    text: String,
    identifier: String,
    final_type: NightObjectType
}

pub trait DefaultObjectMethods {
    type InnerOfCustom;

    fn as_string(&self) -> String;
    fn as_number(&self) -> f64;
    fn as_custom_type(&self) -> Self::InnerOfCustom;

    fn get_text(&self) -> String;
    fn get_identifier(&self) -> String;
    fn get_final_type(&self) -> NightObjectType;
}

impl NightObject {
    pub fn new(
        text: String, identifier: String, final_type: NightObjectType
    ) -> Self {
        NightObject { text, identifier, final_type }
    }
}

impl DefaultObjectMethods for NightObject {
    type InnerOfCustom = String;

    fn as_string(&self) -> String {
        self.text.clone()
    }

    fn as_number(&self) -> f64 {
        match self.text.parse::<f64>() {
            Ok(n) => n,
            Err(n) => spawn_float_error!("Invalid number provided: {n}.")
        }
    }

    // Must be implemented later.
    fn as_custom_type(&self) -> Self::InnerOfCustom {
        todo!()
    }

    fn get_text(&self) -> String {
        self.text.clone()
    }

    fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    fn get_final_type(&self) -> NightObjectType {
        self.final_type.clone()
    }
}

#[cfg(debug_assertions)]
impl Debug for NightObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}: {} = {}", &self.identifier, &self.final_type, &self.text)
    }
}
