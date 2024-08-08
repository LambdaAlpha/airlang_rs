use crate::{
    ctx::CtxValue,
    prelude::{
        named_free_fn,
        Named,
        Prelude,
    },
    FuncVal,
    Map,
    Mode,
    Symbol,
    Text,
    Val,
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
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn(
        "symbol.from_text",
        input_mode,
        output_mode,
        true,
        fn_from_text,
    )
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
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn(
        "symbol.into_text",
        input_mode,
        output_mode,
        true,
        fn_into_text,
    )
}

fn fn_into_text(input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    Val::Text(Text::from(String::from(s)).into())
}
