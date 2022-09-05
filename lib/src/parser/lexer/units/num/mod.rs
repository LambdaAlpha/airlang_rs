#[cfg(test)]
mod test;

use crate::num::{ops::CompleteRound, Complete, Float, Integer};
use regex::Regex;

use super::super::LexerError;
use super::super::Token;
use super::super::UnitLexer;

pub struct NumLexer {
    pattern: Regex,
}

impl NumLexer {
    pub fn new() -> NumLexer {
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
        ([pP](?P<prec>[0-9](?:[0-9_]*[0-9])?))? # precision
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
    fn lexing(&self, captures: &regex::Captures) -> Result<Token, LexerError> {
        let sign = captures.name("sign").unwrap().as_str();
        let sign = if sign == "-" { "-" } else { "+" };

        if let Some(binary) = captures.name("bin") {
            let binary = binary.as_str().replace("_", "");
            let binary = format!("{sign}{binary}");
            return Ok(Token::Int(
                Integer::parse_radix(binary.as_str(), 2).unwrap().complete(),
            ));
        }

        if let Some(hex) = captures.name("hex") {
            let hex = hex.as_str().replace("_", "");
            let hex = format!("{sign}{hex}");
            return Ok(Token::Int(
                Integer::parse_radix(hex.as_str(), 16).unwrap().complete(),
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
            let exp_sign = exp_sign.map_or("+".to_owned(), |m| {
                if m.as_str() == "-" { "-" } else { "+" }.to_owned()
            });
            let num = format!("{sign}{decimal}.{fraction}e{exp_sign}{exponent}");
            // todo default precision algorithm
            let precision = precision.map_or("53".to_owned(), |m| m.as_str().replace("_", ""));
            let precision = u32::from_str_radix(precision.as_str(), 10).unwrap();

            Ok(Token::Float(
                Float::parse(num.as_str()).unwrap().complete(precision),
            ))
        } else {
            let decimal = format!("{sign}{decimal}");
            Ok(Token::Int(
                Integer::parse(decimal.as_str()).unwrap().complete(),
            ))
        }
    }
}

// todo refine float parse
static LOG_2_10: f64 = 3.3219280948873626;
