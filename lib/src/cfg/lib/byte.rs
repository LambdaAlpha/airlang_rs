use const_format::concatcp;

use super::ConstImpl;
use super::FreeImpl;
use super::ImplExtra;
use super::MutImpl;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::BYTE;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::Int;
use crate::type_::Pair;

// todo design add more
#[derive(Clone)]
pub struct ByteLib {
    pub get_length: PrimFuncVal,
    pub push: PrimFuncVal,
    pub join: PrimFuncVal,
}

pub const GET_LENGTH: &str = concatcp!(PREFIX_ID, BYTE, ".get_length");
pub const PUSH: &str = concatcp!(PREFIX_ID, BYTE, ".push");
pub const JOIN: &str = concatcp!(PREFIX_ID, BYTE, ".join");

impl Default for ByteLib {
    fn default() -> Self {
        ByteLib { get_length: get_length(), push: push(), join: join() }
    }
}

impl CfgMod for ByteLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, PUSH, self.push);
        extend_func(cfg, JOIN, self.join);
    }
}

pub fn get_length() -> PrimFuncVal {
    ConstImpl { fn_: fn_get_length }.build(ImplExtra { raw_input: false })
}

fn fn_get_length(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Byte(byte) = ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a byte, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_LENGTH}: expected input to be a unit, but got {input}");
    }
    let len: Int = byte.len().into();
    Val::Int(len.into())
}

pub fn push() -> PrimFuncVal {
    MutImpl { fn_: fn_push }.build(ImplExtra { raw_input: false })
}

fn fn_push(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Byte(byte) = ctx else {
        return bug!(cfg, "{PUSH}: expected context to be a byte, but got {ctx}");
    };
    let Val::Byte(b) = input else {
        return bug!(cfg, "{PUSH}: expected input to be a byte, but got {input}");
    };
    byte.push(&b);
    Val::default()
}

// todo design
pub fn join() -> PrimFuncVal {
    FreeImpl { fn_: fn_join }.build(ImplExtra { raw_input: false })
}

fn fn_join(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{JOIN}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Byte(separator) = pair.left else {
        return bug!(cfg, "{JOIN}: expected input.left to be a byte, but got {}", pair.left);
    };
    let Val::List(bytes) = pair.right else {
        return bug!(cfg, "{JOIN}: expected input.right to be a list, but got {}", pair.right);
    };
    let mut to_join: Vec<&[u8]> = Vec::with_capacity(bytes.len());
    for byte in bytes.iter() {
        let Val::Byte(b) = byte else {
            return bug!(cfg, "{JOIN}: expected input.right.item to be a byte, but got {byte}");
        };
        to_join.push(b);
    }
    let byte = to_join.join(&**separator);
    Val::Byte(Byte::from(byte).into())
}
