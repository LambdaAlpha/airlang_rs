use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Text;

// todo design add more
#[derive(Clone)]
pub struct TextLib {
    pub from_utf8: FreePrimFuncVal,
    pub into_utf8: FreePrimFuncVal,
    pub get_length: ConstPrimFuncVal,
    pub push: MutPrimFuncVal,
    pub join: FreePrimFuncVal,
}

impl Default for TextLib {
    fn default() -> Self {
        TextLib {
            from_utf8: from_utf8(),
            into_utf8: into_utf8(),
            get_length: get_length(),
            push: push(),
            join: join(),
        }
    }
}

impl CfgMod for TextLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_text.from_utf8", self.from_utf8);
        extend_func(cfg, "_text.into_utf8", self.into_utf8);
        extend_func(cfg, "_text.get_length", self.get_length);
        extend_func(cfg, "_text.push", self.push);
        extend_func(cfg, "_text.join", self.join);
    }
}

pub fn from_utf8() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_from_utf8) }.free()
}

fn fn_from_utf8(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Byte(byte) = input else {
        error!("input {input:?} should be a byte");
        return illegal_input(cfg);
    };
    let byte = Byte::from(byte);
    let Ok(str) = String::from_utf8(byte.into()) else {
        error!("input should be a utf8 text");
        return illegal_input(cfg);
    };
    Val::Text(Text::from(str).into())
}

pub fn into_utf8() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_into_utf8) }.free()
}

fn fn_into_utf8(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(text) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    let text = Text::from(text);
    let byte = Byte::from(String::from(text).into_bytes());
    Val::Byte(byte.into())
}

pub fn get_length() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_length) }.const_()
}

fn fn_get_length(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Text(t) = &*ctx else {
        error!("ctx {ctx:?} should be a text");
        return illegal_ctx(cfg);
    };
    let len: Int = t.len().into();
    Val::Int(len.into())
}

pub fn push() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_push) }.mut_()
}

fn fn_push(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Text(text) = ctx else {
        error!("ctx {ctx:?} should be a text");
        return illegal_ctx(cfg);
    };
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    text.push_str(&t);
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
    let separator = match &pair.first {
        Val::Unit(_) => "",
        Val::Text(t) => t,
        v => {
            error!("separator {v:?} should be a text or a unit");
            return illegal_input(cfg);
        }
    };
    let Val::List(texts) = &pair.second else {
        error!("input.second {:?} should be a list", pair.second);
        return illegal_input(cfg);
    };
    let texts: Option<Vec<&str>> = texts
        .iter()
        .map(|v| {
            let Val::Text(t) = v else {
                error!("item {v:?} should be a text");
                return None;
            };
            let s: &str = t;
            Some(s)
        })
        .collect();
    let Some(texts) = texts else {
        return illegal_input(cfg);
    };
    let text = texts.join(separator);
    Val::Text(Text::from(text).into())
}
