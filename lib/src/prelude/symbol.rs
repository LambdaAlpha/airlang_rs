use crate::ConstFnCtx;
use crate::FuncMode;
use crate::FuncVal;
use crate::Int;
use crate::Pair;
use crate::Symbol;
use crate::Text;
use crate::Val;
use crate::ctx::main::MainCtx;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::prelude::ref_pair_mode;

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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.from_text.put(ctx);
        self.into_text.put(ctx);
        self.length.put(ctx);
        self.join.put(ctx);
    }
}

fn from_text() -> Named<FuncVal> {
    let id = "symbol.from_text";
    let f = fn_from_text;
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
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
    named_free_fn(id, f, mode)
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
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_length(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
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
    named_free_fn(id, f, mode)
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
