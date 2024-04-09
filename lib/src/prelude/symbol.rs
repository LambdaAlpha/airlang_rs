use crate::{
    ctx::CtxMap,
    prelude::{
        default_mode,
        named_free_fn,
        Named,
        Prelude,
    },
    FuncVal,
    Str,
    Symbol,
    Val,
};

#[derive(Clone)]
pub(crate) struct SymbolPrelude {
    pub(crate) empty: Named<Symbol>,
    pub(crate) from_str: Named<FuncVal>,
    pub(crate) into_str: Named<FuncVal>,
}

impl Default for SymbolPrelude {
    fn default() -> Self {
        SymbolPrelude {
            empty: empty(),
            from_str: from_str(),
            into_str: into_str(),
        }
    }
}

impl Prelude for SymbolPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.empty.put(m);
        self.from_str.put(m);
        self.into_str.put(m);
    }
}

fn empty() -> Named<Symbol> {
    Named::new("", Symbol::from_str(""))
}

fn from_str() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("symbol.from_string", input_mode, output_mode, fn_from_str)
}

fn fn_from_str(input: Val) -> Val {
    let Val::String(s) = input else {
        return Val::default();
    };
    let is_symbol = s.chars().all(Symbol::is_symbol);
    if !is_symbol {
        return Val::default();
    }
    let symbol = Symbol::from_string(s.to_string());
    Val::Symbol(symbol)
}

fn into_str() -> Named<FuncVal> {
    let input_mode = default_mode();
    let output_mode = default_mode();
    named_free_fn("symbol.into_string", input_mode, output_mode, fn_into_str)
}

fn fn_into_str(input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    Val::String(Str::from(String::from(s)))
}
