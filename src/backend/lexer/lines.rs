use std::fmt::{Display, Formatter, Result};

use crate::spawn_core_error;

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

pub struct Position {
    number: usize,
    column: usize
}

impl Position {
    pub fn new(number: usize, column: usize) -> Self {
        Position {
            number: number + 1, // Cause indexes starts from 0.
            column: column + 1
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}:{}", &self.number, &self.column)
    }
}

pub struct Cursor<'a> {
    pub line: &'a str,
    pub position: Position
}

impl<'a> Cursor<'a> {
    pub fn new(line: &'a str, number: usize, column: usize) -> Self {
        Cursor {
            line,
            position: Position::new(number, column)
        }
    }
}

impl Display for Cursor<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} | {}", &self.position, &self.line)
    }
}

pub struct LineManager {
    code: Vec<String>
}

impl LineManager {
    pub fn new(code_ref: &[u8]) -> Self {
        LineManager {
            code: transform_code(code_ref)
        }
    }

    // dry.
    pub fn get_position(&self, mut column: usize) -> Position {
        let mut number = 0;

        for line in self.code.iter() {
            if column < line.len() {
                return Position::new(number, column);
            }
            else {
                number += 1;
                column -= line.len();
            }
        }

        spawn_core_error!("P: Failed to access the {} index.", column)
    }

    // dry.
    pub fn get_cursor(&self, mut column: usize) -> Cursor<'_> {
        let mut number = 1;

        for line in self.code.iter() {
            if column < line.len() {
                return Cursor::new(line, number, column);
            }
            else {
                number += 1;
                column -= line.len();
            }
        }

        spawn_core_error!("C: Failed to access the {} index.", column)
    }
}
