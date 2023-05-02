//! Should be refactored, since the
//! quality of the code here is low.

use crate::frontend::objects_driver::NightObject;
use crate::spawn_name_error;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};

#[derive(Default)]
pub struct NightObjectsDriver {
    store: HashMap<String, NightObject>
}

impl NightObjectsDriver {
    fn add(&mut self, object: NightObject) {
        self.store.insert(object.identifier.clone(), object); // TODO: Refactor `clone()` calling.
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
