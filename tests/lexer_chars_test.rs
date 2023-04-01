extern crate night;

use night::backend::lexer::{Char, LexicalAssertions};

static NIGHT_REF: [u8; 6] = [
    110, 105, 103, 104, 116, 0
];
static NIGHT_CHARS: [char; 6] = [
    'n', 'i', 'g', 'h', 't', '\0'
];

#[test]
fn constructor_test() {
    let eof_char = Char::new(&0);
    let zero_char = Char::new(&48);

    assert_eq!(eof_char.reference, '\0');
    assert_eq!(zero_char.reference, '0');
    
    let night = NIGHT_REF.iter();

    for (i, code) in night.enumerate() {
        let _char = Char::new(code);
        assert_eq!(_char.reference, NIGHT_CHARS[i]);
    }
}

#[test]
fn lexical_assertions_test() {
    
}
