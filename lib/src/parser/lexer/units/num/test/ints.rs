use crate::num::{Complete, Integer};

use crate::parser::lexer::Token::{self, *};

pub fn expected() -> Vec<Token> {
    vec![
        parse_radix("0", 10),
        parse_radix("00", 10),
        parse_radix("+0", 10),
        parse_radix("-0", 10),
        parse_radix("123", 10),
        parse_radix("+123", 10),
        parse_radix("-123", 10),
        parse_radix("01", 10),
        parse_radix("+01", 10),
        parse_radix("-01", 10),
        parse_radix("0", 16),
        parse_radix("00", 16),
        parse_radix("012345", 16),
        parse_radix("a0b1c2d3", 16),
        parse_radix("0", 2),
        parse_radix("1101", 2),
        parse_radix("11111111222222223333333344444444", 10),
        parse_radix("9999999900000000aaaaaaaabbbbbbbbcccccccc", 16),
        parse_radix("1223334444", 10),
    ]
}

fn parse_radix(s: &str, radix: i32) -> Token {
    Int(Integer::parse_radix(s, radix).unwrap().complete())
}
