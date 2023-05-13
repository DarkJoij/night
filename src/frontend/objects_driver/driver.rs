use crate::frontend::objects_driver::NightObject;
use crate::spawn_name_error;

use std::collections::HashMap;

pub struct NightObjectsDriver {
    store: HashMap<String, NightObject>
}

impl NightObjectsDriver {
    pub fn new() -> Self {
        NightObjectsDriver {
            store: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: String, value: NightObject) {
        self.store.insert(name, value);
    }

    // May be edited without reference return value.
    pub fn get(&self, name: &String) -> &NightObject {
        match self.store.get(name) {
            None => spawn_name_error!("Object with identifier '{name}' is not exists."),
            Some(o) => o,
        }
    }

    pub fn remove(&mut self, name: &String) {
        self.store.remove(name);
    }
}
