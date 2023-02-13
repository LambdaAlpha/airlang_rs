use {
    crate::{
        grammar::stringify::stringify_pretty,
        repr::Repr,
    },
    nom::{
        error::{
            convert_error,
            VerboseError,
        },
        Finish,
    },
    thiserror::Error,
};

mod parse;
mod stringify;
#[cfg(test)]
mod test;

const COMMENT_PREFIX: char = '#';

const PRESERVE_PREFIX: char = '\'';

const SEPARATOR: char = ',';
const PAIR_SEPARATOR: char = ':';
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

pub(crate) type ParseResult<T> = Result<T, ParseError>;

pub(crate) fn parse(src: &str) -> ParseResult<Repr> {
    let ret = parse::parse::<VerboseError<&str>>(src).finish();
    match ret {
        Ok(r) => Ok(r.1),
        Err(e) => {
            let msg = convert_error(src, e);
            Err(ParseError { msg })
        }
    }
}

pub(crate) fn stringify(src: &Repr) -> String {
    stringify_pretty(src)
}
