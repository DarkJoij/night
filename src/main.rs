mod backend;
mod frontend;
mod macros;

use crate::backend::lexer::{LineManager, Lexer};
use crate::frontend::objects_driver::{
    DriverInstruments,
    NightObjectsDriver,
    NightObjectType,
    NightObject
};
use crate::frontend::utils::{Argv, read_file};

fn main() {
    let argv = Argv::default();
    let code = read_file(argv.get(1));
    let line_manager = LineManager::new(&code);

    let mut lexer = Lexer::new(&code, &line_manager);
    let tokens = lexer.lex();

    if_debug! {
        println!("Tokens:\n{tokens:#?}\n");
    }

    if_daily! {
        let zero = NightObject::new(
            String::from("0"),
            String::from("zero"),
            NightObjectType::Number
        );
        let one = NightObject::new(
            String::from("1"),
            String::from("one"),
            NightObjectType::Number
        );

        let mut driver = NightObjectsDriver::default();
        println!("{driver:?}");

        driver.add(zero);
        driver.add(one);

        println!("{driver:?}");
    }
}
