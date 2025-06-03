use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::Int;
use crate::Symbol;
use crate::Text;
use crate::Val;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;

#[derive(Clone)]
pub(crate) struct SymbolPrelude {
    pub(crate) from_text: FreeStaticPrimFuncVal,
    pub(crate) into_text: FreeStaticPrimFuncVal,
    pub(crate) length: ConstStaticPrimFuncVal,
    pub(crate) join: FreeStaticPrimFuncVal,
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

fn from_text() -> FreeStaticPrimFuncVal {
    FreeFn { id: "symbol.from_text", f: free_impl(fn_from_text), mode: FuncMode::default() }
        .free_static()
}

fn fn_from_text(input: Val) -> Val {
    let Val::Text(t) = input else {
        return Val::default();
    };
    let is_symbol = t.chars().all(Symbol::is_symbol);
    if !is_symbol {
        return Val::default();
    }
    let symbol = Symbol::from_string(t.to_string());
    Val::Symbol(symbol)
}

fn into_text() -> FreeStaticPrimFuncVal {
    FreeFn { id: "symbol.into_text", f: free_impl(fn_into_text), mode: FuncMode::default() }
        .free_static()
}

fn fn_into_text(input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    Val::Text(Text::from(String::from(s)).into())
}

fn length() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "symbol.length",
        f: const_impl(fn_length),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Symbol(symbol) = &*ctx else {
        return Val::default();
    };
    let len: Int = symbol.len().into();
    Val::Int(len.into())
}

fn join() -> FreeStaticPrimFuncVal {
    FreeFn { id: "symbol.join", f: free_impl(fn_join), mode: FuncMode::default() }.free_static()
}

fn fn_join(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let separator = match &pair.first {
        Val::Unit(_) => "",
        Val::Symbol(s) => s,
        _ => return Val::default(),
    };
    let Val::List(symbols) = &pair.second else {
        return Val::default();
    };
    let symbols: Option<Vec<&str>> = symbols
        .iter()
        .map(|v| {
            let Val::Symbol(s) = v else {
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
    Val::Symbol(Symbol::from_string(symbol))
}
