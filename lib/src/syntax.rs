use {
    crate::{
        repr::Repr,
        syntax::generator::generate_pretty,
    },
    nom::{
        error::{
            convert_error,
            VerboseError,
        },
        Finish,
    },
    std::{
        fmt::{
            Debug,
            Display,
        },
        str::FromStr,
    },
    thiserror::Error,
};

mod generator;
mod parser;
#[cfg(test)]
mod test;

const COMMENT_PREFIX: char = '#';

const PRESERVE_PREFIX: char = '\'';

const SEPARATOR: char = ',';
const PAIR_SEPARATOR: char = ':';
const INVERSE_SEPARATOR: char = '?';
const LIST_LEFT: char = '(';
const LIST_RIGHT: char = ')';
const MAP_LEFT: char = '{';
const MAP_RIGHT: char = '}';
const WRAP_LEFT: char = '[';
const WRAP_RIGHT: char = ']';

#[derive(Error, Debug)]
#[error("ParseError:\n{msg}")]
pub struct ParseError {
    pub(crate) msg: String,
}

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    let ret = parser::parse::<VerboseError<&str>>(src).finish();
    match ret {
        Ok(r) => Ok(r.1),
        Err(e) => {
            let msg = convert_error(src, e);
            Err(ParseError { msg })
        }
    }
}

pub fn generate(src: &Repr) -> String {
    generate_pretty(src)
}

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate(self))
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate(self))
    }
}

impl TryFrom<&str> for Repr {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse(value)
    }
}

impl FromStr for Repr {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl Into<String> for &Repr {
    fn into(self) -> String {
        generate(self)
    }
}
