use crate::spawn_name_error;
use crate::frontend::objects_driver::object::{
    DefaultObjectMethods, NightObject
};

use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};

#[derive(Default)]
pub struct NightObjectsDriver {
    store: HashMap<String, NightObject>
}

pub trait DriverInstruments {
    fn add(&mut self, object: NightObject);
    fn get(&self, identifier: String) -> &NightObject;
}

impl DriverInstruments for NightObjectsDriver {
    fn add(&mut self, object: NightObject) {
        self.store.insert(object.get_identifier(), object);
    }

    fn get(&self, identifier: String) -> &NightObject {
        match self.store.get(&identifier) {
            Some(o) => o,
            None => spawn_name_error!("Invalid identifier provided: {}.", identifier)
        }
    }
}

#[cfg(debug_assertions)]
impl Debug for NightObjectsDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.store.is_empty() {
            return writeln!(f, "Empty store.");
        }

        for object in self.store.values() {
            writeln!(f, "{object:?}")?;
        }

        write!(f, "end.")
    }
}

#[cfg(feature = "daily")]
impl NightObjectsDriver {
    pub fn del(&mut self, identifier: String) {
        self.store.remove(&identifier);
    }
}
