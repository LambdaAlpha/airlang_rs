use const_format::concatcp;
use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::BYTE;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Pair;

// todo design add more
#[derive(Clone)]
pub struct ByteLib {
    pub get_length: ConstPrimFuncVal,
    pub push: MutPrimFuncVal,
    pub join: FreePrimFuncVal,
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

pub fn get_length() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_length) }.const_()
}

fn fn_get_length(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Byte(byte) = &*ctx else {
        error!("ctx {ctx:?} should be a byte");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let len: Int = byte.len().into();
    Val::Int(len.into())
}

pub fn push() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_push) }.mut_()
}

fn fn_push(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Byte(byte) = ctx else {
        error!("ctx {ctx:?} should be a byte");
        return illegal_ctx(cfg);
    };
    let Val::Byte(b) = input else {
        error!("input {input:?} should be a byte");
        return illegal_input(cfg);
    };
    byte.push(&b);
    Val::default()
}

// todo design
pub fn join() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_join) }.free()
}

fn fn_join(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Byte(separator) = pair.left else {
        error!("separator {:?} should be a byte", pair.left);
        return illegal_input(cfg);
    };
    let Val::List(bytes) = pair.right else {
        error!("input.right {:?} should be a list", pair.right);
        return illegal_input(cfg);
    };
    let bytes: Option<Vec<&[u8]>> = bytes
        .iter()
        .map(|v| {
            let Val::Byte(b) = v else {
                error!("item {v:?} should be a byte");
                return None;
            };
            let byte: &[u8] = b;
            Some(byte)
        })
        .collect();
    let Some(bytes) = bytes else {
        return illegal_input(cfg);
    };
    let byte = bytes.join(&**separator);
    Val::Byte(Byte::from(byte).into())
}
