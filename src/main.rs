mod backend;
mod frontend;
mod macros;

use crate::backend::lexer::{LineManager, Lexer};
use crate::backend::parser::Parser;
use crate::frontend::objects_driver::get_loaded_driver;
use crate::frontend::utils::{Argv, read_file};

fn main() {
    let argv = Argv::default();
    let code = read_file(argv.get(1));
    let line_manager = LineManager::new(&code);

    let mut lexer = Lexer::new(&code, &line_manager);
    let tokens = lexer.lex();

    // println!("Tokens:\n{tokens:#?}\n");

    let mut driver = get_loaded_driver();
    let mut parser = Parser::new(&tokens, &line_manager);
    let statements = parser.parse();

    for statement in statements {
        statement.execute(&mut driver)
    }
}
