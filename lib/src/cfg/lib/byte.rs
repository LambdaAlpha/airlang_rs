use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::ConstRef;
use crate::type_::Int;

// todo design add more
#[derive(Clone)]
pub struct ByteLib {
    pub length: ConstPrimFuncVal,
    pub push: MutPrimFuncVal,
    pub join: FreePrimFuncVal,
}

impl Default for ByteLib {
    fn default() -> Self {
        ByteLib { length: length(), push: push(), join: join() }
    }
}

impl CfgMod for ByteLib {
    fn extend(self, cfg: &Cfg) {
        self.length.extend(cfg);
        self.push.extend(cfg);
        self.join.extend(cfg);
    }
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "_byte.length", raw_input: false, f: const_impl(fn_length) }.const_()
}

fn fn_length(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Byte(byte) = &*ctx else {
        error!("ctx {ctx:?} should be a byte");
        return illegal_ctx(cfg);
    };
    let len: Int = byte.len().into();
    Val::Int(len.into())
}

pub fn push() -> MutPrimFuncVal {
    DynPrimFn { id: "_byte.push", raw_input: false, f: mut_impl(fn_push) }.mut_()
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
    FreePrimFn { id: "_byte.join", raw_input: false, f: free_impl(fn_join) }.free()
}

fn fn_join(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let separator: &[u8] = match &pair.first {
        Val::Unit(_) => &[],
        Val::Byte(b) => b,
        s => {
            error!("separator {s:?} should be a unit or a byte");
            return illegal_input(cfg);
        }
    };
    let Val::List(bytes) = &pair.second else {
        error!("input.second {:?} should be a list", pair.second);
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
    let byte = bytes.join(separator);
    Val::Byte(Byte::from(byte).into())
}
