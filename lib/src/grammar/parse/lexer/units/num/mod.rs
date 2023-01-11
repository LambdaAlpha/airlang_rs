use {
    crate::{
        grammar::{
            parse::lexer::{
                Token,
                UnitLexer,
            },
            repr::{
                Float,
                Int,
            },
            ParseResult,
        },
        utils,
    },
    regex::Regex,
};

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
        1[bB](?P<bytes_bin>(?:[01]{8}|_)*)
        |
        1[xX](?P<bytes_hex>(?:[0-9a-fA-F]{2}|_)*)
        |
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
        let bytes_bin = captures.name("bytes_bin");
        if bytes_bin.is_some() {
            let bin = bytes_bin.unwrap().as_str().replace("_", "");
            let bytes = utils::conversion::bin_str_to_vec_u8(bin.as_str()).unwrap();
            return Ok(Token::Bytes(bytes));
        }

        let bytes_hex = captures.name("bytes_hex");
        if bytes_hex.is_some() {
            let hex = bytes_hex.unwrap().as_str().replace("_", "");
            let bytes = utils::conversion::hex_str_to_vec_u8(hex.as_str()).unwrap();
            return Ok(Token::Bytes(bytes));
        }

        let sign = captures.name("sign").unwrap().as_str();
        let sign = sign != "-";

        if let Some(binary) = captures.name("bin") {
            let binary = binary.as_str().replace("_", "");
            return Ok(Token::Int(Int::new(sign, 2, binary)));
        }

        if let Some(hex) = captures.name("hex") {
            let hex = hex.as_str().replace("_", "");
            return Ok(Token::Int(Int::new(sign, 16, hex)));
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
            let exp_sign = exp_sign.map_or(true, |m| m.as_str() != "-");

            Ok(Token::Float(Float::new(
                sign, decimal, fraction, exp_sign, exponent,
            )))
        } else {
            Ok(Token::Int(Int::new(sign, 10, decimal)))
        }
    }
}
