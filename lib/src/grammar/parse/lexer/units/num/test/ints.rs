use crate::grammar::repr::Int;

use super::super::Token::{self, *};

pub(crate) fn expected() -> Vec<Token> {
    vec![
        Symbol("+".to_owned()),
        Symbol("-".to_owned()),
        int(true, "0", 10),
        int(true, "00", 10),
        int(true, "0", 10),
        int(false, "0", 10),
        int(true, "123", 10),
        int(true, "123", 10),
        int(false, "123", 10),
        int(true, "01", 10),
        int(true, "01", 10),
        int(false, "01", 10),
        int(true, "0", 16),
        int(true, "00", 16),
        int(true, "012345", 16),
        int(true, "a0b1c2d3", 16),
        int(true, "0", 2),
        int(true, "1101", 2),
        int(true, "11111111222222223333333344444444", 10),
        int(true, "9999999900000000aaaaaaaabbbbbbbbcccccccc", 16),
        int(true, "1223334444", 10),
    ]
}

fn int(sign: bool, s: &str, radix: u8) -> Token {
    Int(Int::new(sign, radix, s.to_owned()))
}
