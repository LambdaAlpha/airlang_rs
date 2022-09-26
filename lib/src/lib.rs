use val::{Bytes, Val};

pub fn interpret(src: &str) -> String {
    // todo impl
    return src.to_string();
}

pub fn parse(src: &str) -> Val {
    let result = grammar::parse(src);
    match result {
        Ok(val) => val,
        Err(_) => Val::from(vec![] as Bytes),
    }
}

pub fn eval(src: Val) -> Val {
    // todo impl
    return src;
}

pub mod val;

mod grammar;
#[allow(dead_code)]
mod utils;
use rug as num;
