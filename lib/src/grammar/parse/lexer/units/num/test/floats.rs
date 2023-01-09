use crate::grammar::{
    parse::lexer::Token,
    repr::Float,
};

pub(crate) fn expected() -> Vec<Token> {
    vec![
        float(true, "0", "0", true, "0"),
        float(true, "0", "0", true, "0"),
        float(false, "0", "0", true, "0"),
        float(true, "0", "", true, "0"),
        float(true, "1", "", true, "0"),
        float(true, "1", "", true, "2"),
        float(false, "123", "455666", false, "123"),
        float(true, "1", "111111111111111111111111111111", true, "100"),
    ]
}

fn float(sign: bool, integral: &str, fractional: &str, exp_sign: bool, exp_digits: &str) -> Token {
    Token::Float(Float::new(
        sign,
        integral.to_owned(),
        fractional.to_owned(),
        exp_sign,
        exp_digits.to_owned(),
    ))
}
