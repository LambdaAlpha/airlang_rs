use regex::Regex;

use crate::grammar::repr::{Float, Int};

use super::super::{ParseResult, Token, UnitLexer};

#[cfg(test)]
mod test;

pub(crate) struct NumLexer {
    pattern: Regex,
}

impl NumLexer {
    pub(crate) fn new() -> NumLexer {
        NumLexer {
            pattern: Regex::new(
                r"(?x)
        (?P<sign>[+\\-]?)
        (?:
        0[bB](?P<bin>[01](?:[01_]*[01])?) # binary
        |
        0[xX](?P<hex>[0-9a-fA-F](?:[0-9a-fA-F_]*[0-9a-fA-F])?) # hex
        |
        (?P<dec>[0-9](?:[0-9_]*[0-9])?) # decimal
        ((?P<point>\.)(?P<frac>[0-9](?:[0-9_]*[0-9])?)?)? # fraction
        ([eE](?P<exp_sign>[+\\-]?)(?P<exp>[0-9](?:[0-9_]*[0-9])?))? #exponent
        )
        ",
            )
                .unwrap(),
        }
    }
}

impl UnitLexer for NumLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> ParseResult<Token> {
        let sign = captures.name("sign").unwrap().as_str();
        let sign = sign != "-";

        if let Some(binary) = captures.name("bin") {
            let binary = binary.as_str().replace("_", "");
            return Ok(Token::Int(
                Int::new(sign, 2, binary),
            ));
        }

        if let Some(hex) = captures.name("hex") {
            let hex = hex.as_str().replace("_", "");
            return Ok(Token::Int(
                Int::new(sign, 16, hex),
            ));
        }

        let decimal = captures.name("dec").unwrap().as_str().replace("_", "");
        let point = captures.name("point");
        let fraction = captures.name("frac");
        let exponent = captures.name("exp");
        let exp_sign = captures.name("exp_sign");
        let precision = captures.name("prec");

        if point != None || exponent != None || precision != None {
            let fraction = fraction.map_or("".to_owned(), |m| m.as_str().replace("_", ""));
            let exponent = exponent.map_or("0".to_owned(), |m| m.as_str().replace("_", ""));
            let exp_sign = exp_sign.map_or(true, |m| {
                m.as_str() != "-"
            });

            Ok(Token::Float(
                Float::new(sign, decimal, fraction, exp_sign, exponent)
            ))
        } else {
            Ok(Token::Int(
                Int::new(sign, 10, decimal)
            ))
        }
    }
}
