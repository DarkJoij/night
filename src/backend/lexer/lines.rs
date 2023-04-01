use crate::spawn_night_error;
use crate::backend::defaults::IGNORE_CHARS;

fn is_last(last: &[char], symbol: &char) -> bool {
    symbol == last.last()
        .unwrap()
}

// It's not memory-safe, and must be refactored!
unsafe fn transform_code(code_ref: &[u8]) -> Vec<String> {
    let mut lines = Vec::new();
    
    let mut buffer = String::new();
    let code: Vec<char> = code_ref.iter()
        .map(|b| (*b) as char)
        .collect();
    
    for char_link in &code {
        let character = *char_link;

        if IGNORE_CHARS.contains(char_link) {
            if character == '\n' || is_last(&code, char_link) {
                buffer.push(' ');
                
                lines.push(buffer.clone());
                buffer.clear();

                continue;
            }

            buffer.push(' ');
        }
        else {
            buffer.push(character);
        }
    }

    lines
}

pub struct LineManager {
    code: Vec<String>
}

impl LineManager {
    pub fn new(code_ref: &[u8]) -> Self {
        let lm = LineManager {
            code: unsafe {
                transform_code(code_ref)
            }
        };

        println!("{:#?}", lm.code);

        lm
    }

    pub fn get_current_line(&self, position: &usize) -> &String {
        let mut inner_position = *position;

        for line in self.code.iter() {
            if inner_position < line.len() {
                return line
            }
            else {
                inner_position -= line.len();
            }
        }

        spawn_night_error!("Failed to access the {} index.", position)
    }
}
