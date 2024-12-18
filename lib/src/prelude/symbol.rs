use crate::{
    FuncMode,
    FuncVal,
    Map,
    Mode,
    Symbol,
    Text,
    Val,
    ctx::CtxValue,
    prelude::{
        Named,
        Prelude,
        named_free_fn,
    },
};

#[derive(Clone)]
pub(crate) struct SymbolPrelude {
    pub(crate) from_text: Named<FuncVal>,
    pub(crate) into_text: Named<FuncVal>,
}

impl Default for SymbolPrelude {
    fn default() -> Self {
        SymbolPrelude {
            from_text: from_text(),
            into_text: into_text(),
        }
    }
}

impl Prelude for SymbolPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.from_text.put(m);
        self.into_text.put(m);
    }
}

fn from_text() -> Named<FuncVal> {
    let id = "symbol.from_text";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    let f = fn_from_text;
    named_free_fn(id, mode, cacheable, f)
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

fn into_text() -> Named<FuncVal> {
    let id = "symbol.into_text";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    let f = fn_into_text;
    named_free_fn(id, mode, cacheable, f)
}

fn fn_into_text(input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    Val::Text(Text::from(String::from(s)).into())
}
