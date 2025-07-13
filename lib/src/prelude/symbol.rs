use log::error;

use super::DynFn;
use super::FreeFn;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::free_impl;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Symbol;
use crate::type_::Text;

// todo design add more
#[derive(Clone)]
pub struct SymbolPrelude {
    pub from_text: FreeStaticPrimFuncVal,
    pub into_text: FreeStaticPrimFuncVal,
    pub length: ConstStaticPrimFuncVal,
    pub join: FreeStaticPrimFuncVal,
}

impl Default for SymbolPrelude {
    fn default() -> Self {
        SymbolPrelude {
            from_text: from_text(),
            into_text: into_text(),
            length: length(),
            join: join(),
        }
    }
}

impl Prelude for SymbolPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.from_text.put(ctx);
        self.into_text.put(ctx);
        self.length.put(ctx);
        self.join.put(ctx);
    }
}

pub fn from_text() -> FreeStaticPrimFuncVal {
    FreeFn { id: "symbol.from_text", f: free_impl(fn_from_text), mode: default_free_mode() }
        .free_static()
}

fn fn_from_text(input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    let is_symbol = t.chars().all(Symbol::is_symbol);
    if !is_symbol {
        error!("every character of input {t:?} text should be a symbol");
        return Val::default();
    }
    let symbol = Symbol::from_string_unchecked(t.to_string());
    Val::Symbol(symbol)
}

pub fn into_text() -> FreeStaticPrimFuncVal {
    FreeFn { id: "symbol.into_text", f: free_impl(fn_into_text), mode: default_free_mode() }
        .free_static()
}

fn fn_into_text(input: Val) -> Val {
    let Val::Symbol(s) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    Val::Text(Text::from(String::from(s)).into())
}

pub fn length() -> ConstStaticPrimFuncVal {
    DynFn { id: "symbol.length", f: const_impl(fn_length), mode: default_dyn_mode() }.const_static()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Symbol(symbol) = &*ctx else {
        error!("ctx {ctx:?} should be a symbol");
        return Val::default();
    };
    let len: Int = symbol.len().into();
    Val::Int(len.into())
}

// todo design
pub fn join() -> FreeStaticPrimFuncVal {
    FreeFn { id: "symbol.join", f: free_impl(fn_join), mode: default_free_mode() }.free_static()
}

fn fn_join(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let separator = match &pair.first {
        Val::Unit(_) => "",
        Val::Symbol(s) => s,
        s => {
            error!("separator {s:?} should be a unit or a symbol");
            return Val::default();
        }
    };
    let Val::List(symbols) = &pair.second else {
        error!("input.second {:?} should be a list", pair.second);
        return Val::default();
    };
    let symbols: Option<Vec<&str>> = symbols
        .iter()
        .map(|v| {
            let Val::Symbol(s) = v else {
                error!("item {v:?} should be a symbol");
                return None;
            };
            let symbol: &str = s;
            Some(symbol)
        })
        .collect();
    let Some(symbols) = symbols else {
        return Val::default();
    };
    let symbol = symbols.join(separator);
    Val::Symbol(Symbol::from_string_unchecked(symbol))
}
