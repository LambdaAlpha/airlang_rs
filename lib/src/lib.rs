use val::Val;

pub fn interpret(src: &str) -> String {
    // todo impl
    return src.to_string();
}

pub fn parse(src: &str) -> Val {
    let result = parser::parse(src);
    match result {
        Ok(val) => val,
        Err(_) => Val::bytes(vec![]),
    }
}

pub fn eval(src: Val) -> Val {
    // todo impl
    return src;
}

pub mod val;

mod parser;
#[allow(dead_code)]
mod utils;
use rug as num;
