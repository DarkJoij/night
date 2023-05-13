use crate::frontend::objects_driver::NightValue;
use crate::spawn_name_error;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};

pub struct NightObjectsDriver {
    store: HashMap<String, NightValue>
}

impl NightObjectsDriver {
    pub fn new() -> Self {
        NightObjectsDriver {
            store: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: String, value: NightValue) {
        self.store.insert(name, value);
    }

    // May be edited without reference return value.
    pub fn get(&self, name: &String) -> NightValue {
        // Without calling [`self.exists`].
        match self.store.get(name) {
            None => spawn_name_error!("Object with identifier '{name}' is not exists."),
            Some(o) => o.clone(),
        }
    }

    pub fn remove(&mut self, name: &String) {
        self.store.remove(name);
    }
}

#[cfg(debug_assertions)]
impl Debug for NightObjectsDriver {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.store.fmt(f)
    }
}

pub fn get_loaded_driver() -> NightObjectsDriver {
    let mut driver = NightObjectsDriver::new();

    driver.insert(
        String::from("HELLO_WORLD"),
        NightValue::Literal(String::from("Hello world!"))
    );

    driver
}
