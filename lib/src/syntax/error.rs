use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
};

pub struct ParseError {
    pub(crate) msg: String,
}

#[derive(Debug)]
pub struct ReprError {}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError\n{}", self.msg)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError\n{}", self.msg)
    }
}

impl Error for ParseError {}

impl Display for ReprError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReprError")
    }
}

impl Error for ReprError {}
