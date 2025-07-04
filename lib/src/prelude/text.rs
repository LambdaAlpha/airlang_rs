use log::error;

use super::DynFn;
use super::FreeFn;
use super::FuncMode;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use super::setup::ctx_default_mode;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Byte;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Text;

// todo design add more
#[derive(Clone)]
pub struct TextPrelude {
    pub from_utf8: FreeStaticPrimFuncVal,
    pub into_utf8: FreeStaticPrimFuncVal,
    pub length: ConstStaticPrimFuncVal,
    pub push: MutStaticPrimFuncVal,
    pub join: FreeStaticPrimFuncVal,
}

impl Default for TextPrelude {
    fn default() -> Self {
        TextPrelude {
            from_utf8: from_utf8(),
            into_utf8: into_utf8(),
            length: length(),
            push: push(),
            join: join(),
        }
    }
}

impl Prelude for TextPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.from_utf8.put(ctx);
        self.into_utf8.put(ctx);
        self.length.put(ctx);
        self.push.put(ctx);
        self.join.put(ctx);
    }
}

pub fn from_utf8() -> FreeStaticPrimFuncVal {
    FreeFn { id: "text.from_utf8", f: free_impl(fn_from_utf8), mode: FuncMode::default() }
        .free_static()
}

fn fn_from_utf8(input: Val) -> Val {
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

pub fn into_utf8() -> FreeStaticPrimFuncVal {
    FreeFn { id: "text.into_utf8", f: free_impl(fn_into_utf8), mode: FuncMode::default() }
        .free_static()
}

fn fn_into_utf8(input: Val) -> Val {
    let Val::Text(text) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    let text = Text::from(text);
    let byte = Byte::from(String::from(text).into_bytes());
    Val::Byte(byte.into())
}

pub fn length() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "text.length",
        f: const_impl(fn_length),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Text(t) = &*ctx else {
        error!("ctx {ctx:?} should be a text");
        return Val::default();
    };
    let len: Int = t.len().into();
    Val::Int(len.into())
}

pub fn push() -> MutStaticPrimFuncVal {
    DynFn {
        id: "text.push",
        f: mut_impl(fn_push),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_push(ctx: &mut Val, input: Val) -> Val {
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
pub fn join() -> FreeStaticPrimFuncVal {
    FreeFn { id: "text.join", f: free_impl(fn_join), mode: FuncMode::default() }.free_static()
}

fn fn_join(input: Val) -> Val {
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
