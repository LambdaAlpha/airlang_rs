use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::mode::FuncMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::memo::Memo;
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

impl Library for ByteLib {
    fn prelude(&self, _memo: &mut Memo) {}
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "byte.length", f: const_impl(fn_length), mode: FuncMode::default_mode() }
        .const_()
}

fn fn_length(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Byte(byte) = &*ctx else {
        error!("ctx {ctx:?} should be a byte");
        return Val::default();
    };
    let len: Int = byte.len().into();
    Val::Int(len.into())
}

pub fn push() -> MutPrimFuncVal {
    DynPrimFn { id: "byte.push", f: mut_impl(fn_push), mode: FuncMode::default_mode() }.mut_()
}

fn fn_push(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Byte(byte) = ctx else {
        error!("ctx {ctx:?} should be a byte");
        return Val::default();
    };
    let Val::Byte(b) = input else {
        error!("input {input:?} should be a byte");
        return Val::default();
    };
    byte.push(&b);
    Val::default()
}

// todo design
pub fn join() -> FreePrimFuncVal {
    FreePrimFn { id: "byte.join", f: free_impl(fn_join), mode: FuncMode::default_mode() }.free()
}

fn fn_join(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let separator: &[u8] = match &pair.first {
        Val::Unit(_) => &[],
        Val::Byte(b) => b,
        s => {
            error!("separator {s:?} should be a unit or a byte");
            return Val::default();
        }
    };
    let Val::List(bytes) = &pair.second else {
        error!("input.second {:?} should be a list", pair.second);
        return Val::default();
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
        return Val::default();
    };
    let byte = bytes.join(separator);
    Val::Byte(Byte::from(byte).into())
}
