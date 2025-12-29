use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::Text;

// todo design add more
#[derive(Clone)]
pub struct KeyLib {
    pub from_text: FreePrimFuncVal,
    pub into_text: FreePrimFuncVal,
    pub length: ConstPrimFuncVal,
    pub join: FreePrimFuncVal,
}

impl Default for KeyLib {
    fn default() -> Self {
        KeyLib { from_text: from_text(), into_text: into_text(), length: length(), join: join() }
    }
}

impl CfgMod for KeyLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_key.from_text", self.from_text);
        extend_func(cfg, "_key.into_text", self.into_text);
        extend_func(cfg, "_key.length", self.length);
        extend_func(cfg, "_key.join", self.join);
    }
}

pub fn from_text() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_from_text) }.free()
}

fn fn_from_text(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    let is_key = t.chars().all(Key::is_key);
    if !is_key {
        error!("every character of input {t:?} text should be a key");
        return illegal_input(cfg);
    }
    let key = Key::from_string_unchecked(t.to_string());
    Val::Key(key)
}

pub fn into_text() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_into_text) }.free()
}

fn fn_into_text(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(key) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    Val::Text(Text::from(String::from(key)).into())
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_length) }.const_()
}

fn fn_length(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Key(key) = &*ctx else {
        error!("ctx {ctx:?} should be a key");
        return illegal_ctx(cfg);
    };
    let len: Int = key.len().into();
    Val::Int(len.into())
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
        Val::Key(key) => key,
        s => {
            error!("separator {s:?} should be a unit or a key");
            return illegal_input(cfg);
        }
    };
    let Val::List(keys) = &pair.second else {
        error!("input.second {:?} should be a list", pair.second);
        return illegal_input(cfg);
    };
    let keys: Option<Vec<&str>> = keys
        .iter()
        .map(|v| {
            let Val::Key(s) = v else {
                error!("item {v:?} should be a key");
                return None;
            };
            let key: &str = s;
            Some(key)
        })
        .collect();
    let Some(keys) = keys else {
        return illegal_input(cfg);
    };
    let key = keys.join(separator);
    Val::Key(Key::from_string_unchecked(key))
}
