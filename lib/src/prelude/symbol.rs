use crate::{
    ConstFnCtx,
    FuncMode,
    FuncVal,
    Int,
    Map,
    Pair,
    Symbol,
    Text,
    Val,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        ref_pair_mode,
    },
};

#[derive(Clone)]
pub(crate) struct SymbolPrelude {
    pub(crate) from_text: Named<FuncVal>,
    pub(crate) into_text: Named<FuncVal>,
    pub(crate) length: Named<FuncVal>,
    pub(crate) join: Named<FuncVal>,
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.from_text.put(m);
        self.into_text.put(m);
        self.length.put(m);
        self.join.put(m);
    }
}

fn from_text() -> Named<FuncVal> {
    let id = "symbol.from_text";
    let f = fn_from_text;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
    let f = fn_into_text;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_into_text(input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    Val::Text(Text::from(String::from(s)).into())
}

fn length() -> Named<FuncVal> {
    let id = "symbol.length";
    let f = fn_length;
    let call = ref_pair_mode();
    let optimize = call.clone();
    let solve = FuncMode::default_mode();
    let mode = FuncMode { call, optimize, solve };
    let cacheable = true;
    named_const_fn(id, f, mode, cacheable)
}

fn fn_length(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Symbol(symbol) = val else {
            return Val::default();
        };
        let len: Int = symbol.len().into();
        Val::Int(len.into())
    })
}

fn join() -> Named<FuncVal> {
    let id = "symbol.join";
    let f = fn_join;
    let mode = FuncMode::default();
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
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
