mod backend;
mod frontend;
mod macros;

use crate::backend::lexer::{Cursor, Lexer};
use crate::frontend::utils::{Argv, read_file};

fn main() {
    let argv = Argv::default();
    let code = read_file(argv.get(1));
    let cursor = Cursor::new(&code);

    let mut lexer = Lexer::new(&code, &cursor);
    let tokens = lexer.lex();

    if_debug! {
        println!("Tokens:\n{tokens:#?}\n");
    }
}
