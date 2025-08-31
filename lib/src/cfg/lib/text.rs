use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
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
    pub length: ConstPrimFuncVal,
    pub push: MutPrimFuncVal,
    pub join: FreePrimFuncVal,
}

impl Default for TextLib {
    fn default() -> Self {
        TextLib {
            from_utf8: from_utf8(),
            into_utf8: into_utf8(),
            length: length(),
            push: push(),
            join: join(),
        }
    }
}

impl CfgMod for TextLib {
    fn extend(self, cfg: &Cfg) {
        self.from_utf8.extend(cfg);
        self.into_utf8.extend(cfg);
        self.length.extend(cfg);
        self.push.extend(cfg);
        self.join.extend(cfg);
    }
}

impl Library for TextLib {
    fn prelude(&self, _ctx: &mut Ctx) {}
}

pub fn from_utf8() -> FreePrimFuncVal {
    FreePrimFn { id: "text.from_utf8", f: free_impl(fn_from_utf8), mode: default_free_mode() }
        .free()
}

fn fn_from_utf8(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Byte(byte) = input else {
        error!("input {input:?} should be a byte");
        return Val::default();
    };
    let byte = Byte::from(byte);
    let Ok(str) = String::from_utf8(byte.into()) else {
        error!("input should be a utf8 text");
        return Val::default();
    };
    Val::Text(Text::from(str).into())
}

pub fn into_utf8() -> FreePrimFuncVal {
    FreePrimFn { id: "text.into_utf8", f: free_impl(fn_into_utf8), mode: default_free_mode() }
        .free()
}

fn fn_into_utf8(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(text) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    let text = Text::from(text);
    let byte = Byte::from(String::from(text).into_bytes());
    Val::Byte(byte.into())
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "text.length", f: const_impl(fn_length), mode: default_dyn_mode() }.const_()
}

fn fn_length(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Text(t) = &*ctx else {
        error!("ctx {ctx:?} should be a text");
        return Val::default();
    };
    let len: Int = t.len().into();
    Val::Int(len.into())
}

pub fn push() -> MutPrimFuncVal {
    DynPrimFn { id: "text.push", f: mut_impl(fn_push), mode: default_dyn_mode() }.mut_()
}

fn fn_push(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Text(text) = ctx else {
        error!("ctx {ctx:?} should be a text");
        return Val::default();
    };
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    text.push_str(&t);
    Val::default()
}

// todo design
pub fn join() -> FreePrimFuncVal {
    FreePrimFn { id: "text.join", f: free_impl(fn_join), mode: default_free_mode() }.free()
}

fn fn_join(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let separator = match &pair.first {
        Val::Unit(_) => "",
        Val::Text(t) => t,
        v => {
            error!("separator {v:?} should be a text or a unit");
            return Val::default();
        }
    };
    let Val::List(texts) = &pair.second else {
        error!("input.second {:?} should be a list", pair.second);
        return Val::default();
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
        return Val::default();
    };
    let text = texts.join(separator);
    Val::Text(Text::from(text).into())
}
