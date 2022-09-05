use val::Val;

pub fn interpret(src: &str) -> String {
    // todo impl
    return src.to_string();
}

pub fn parse(src: &str) -> Val {
    // todo impl
    return Val::Bytes(Box::new(src.as_bytes().to_vec()));
}

pub fn eval(src: Val) -> Val {
    // todo impl
    return src;
}

pub mod val;

#[allow(dead_code)]
mod parser;
#[allow(dead_code)]
mod utils;
use rug as num;
