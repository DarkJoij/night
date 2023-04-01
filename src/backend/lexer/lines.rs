use std::fmt::{Display, Formatter, Result};

use crate::spawn_core_error;
use crate::backend::lexer::Position;

fn is_last(last: &[char], symbol: &char) -> bool {
    symbol == last.last()
        .unwrap()
}

// It's not memory-safe and must be refactored!
fn transform_code(code_ref: &[u8]) -> Vec<String> {
    let mut lines = Vec::new();

    let mut buffer = String::new();
    let code: Vec<char> = code_ref.iter()
        .map(|b| (*b) as char)
        .collect();

    for char_link in &code {
        let character = *char_link;

        if character == '\n' || is_last(&code, char_link) {
            buffer.push(' ');

            lines.push(buffer.clone());
            buffer.clear();

            continue;
        }
        else if character == '\r' {
            buffer.push(' ');
        }
        else {
            buffer.push(character);
        }
    }

    lines
}

pub struct Line<'a> {
    number: usize,
    text: &'a str
}

impl<'a> Line<'a> {
    pub fn new(number: usize, text: &'a str) -> Self {
        Line { number, text: text.trim() }
    }
}

impl Display for Line<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Here it is also necessary to apply
        // color formatting in the future.
        write!(f, "{} | {}", &self.number, &self.text)
    }
}

pub struct Cursor {
    code: Vec<String>
}

impl Cursor {
    pub fn new(code_ref: &[u8]) -> Self {
        Cursor {
            code: transform_code(code_ref)
        }
    }

    pub fn get_current(&self, position: &usize) -> (Position, Line<'_>) {
        let mut line_number = 1;
        let mut c_position = *position;

        for line in self.code.iter() {
            if c_position < line.len() {
                return (
                    Position::new(line_number, c_position),
                    Line::new(line_number, line)
                );
            }
            else {
                line_number += 1;
                c_position -= line.len();
            }
        }

        spawn_core_error!("Failed to access the {} index.", position)
    }
}
