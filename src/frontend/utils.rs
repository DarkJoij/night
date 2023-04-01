use crate::spawn_night_error;

use std::env::args;
use std::fs::read;
use std::process::exit;

// Later it will be used to manage command line arguments.
pub struct Argv {
    reference: Vec<String>
}

impl Argv {
    pub fn get(&self, index: usize) -> &String {
        self.reference.get(index)
            .unwrap()
    }
}

impl Default for Argv {
    fn default() -> Self {
        let argv: Vec<String> = args()
            .collect();

        if argv.len() == 1 {
            spawn_night_error!("Too few arguments passed. At least 2 were expected.");
        }

        Argv { reference: argv }
    }
}

pub fn read_file(path: &String) -> Vec<u8> {
    match read(path.clone()) {
        Ok(c) => {
            if c.is_empty() {
                println!("Code is empty!\nActually task successfully solved.");
                exit(0);
            }

            c
        },
        Err(_) => spawn_night_error!("Failed to read file: {path}.")
    }
}
