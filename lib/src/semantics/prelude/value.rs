use crate::{
    semantics::{
        ctx::{
            DefaultCtx,
            NameMap,
        },
        ctx_access::constant::CtxForConstFn,
        eval_mode::EvalMode,
        input_mode::InputMode,
        prelude::{
            named_const_fn,
            Named,
            Prelude,
        },
        val::FuncVal,
        Val,
    },
    types::{
        Bool,
        Pair,
        Symbol,
    },
};

#[derive(Clone)]
pub(crate) struct ValuePrelude {
    type_of: Named<FuncVal>,
    equal: Named<FuncVal>,
    not_equal: Named<FuncVal>,
}

impl Default for ValuePrelude {
    fn default() -> Self {
        ValuePrelude {
            type_of: type_of(),
            equal: equal(),
            not_equal: not_equal(),
        }
    }
}

impl Prelude for ValuePrelude {
    fn put(&self, m: &mut NameMap) {
        self.type_of.put(m);
        self.equal.put(m);
        self.not_equal.put(m);
    }
}

fn type_of() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("type_of", input_mode, fn_type_of)
}

fn fn_type_of(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let s = match val {
            Val::Unit(_) => "unit",
            Val::Bool(_) => "bool",
            Val::Int(_) => "int",
            Val::Float(_) => "float",
            Val::Bytes(_) => "bytes",
            Val::Symbol(_) => "symbol",
            Val::String(_) => "string",
            Val::Pair(_) => "pair",
            Val::Call(_) => "call",
            Val::Reverse(_) => "reverse",
            Val::List(_) => "list",
            Val::Map(_) => "map",
            Val::Func(_) => "function",
            Val::Ctx(_) => "context",
            Val::Prop(_) => "proposition",
        };
        Val::Symbol(Symbol::from_str(s))
    })
}

fn equal() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Symbol(EvalMode::Value),
    )));
    named_const_fn("==", input_mode, fn_equal)
}

fn fn_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    DefaultCtx.get_many_const_ref(&ctx, [pair.first, pair.second], |[v1, v2]| {
        let eq = if let Ok(v1) = v1
            && let Ok(v2) = v2
        {
            v1 == v2
        } else {
            true
        };
        Val::Bool(Bool::new(eq))
    })
}

fn not_equal() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Symbol(EvalMode::Value),
    )));
    named_const_fn("=/=", input_mode, fn_not_equal)
}

fn fn_not_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    DefaultCtx.get_many_const_ref(&ctx, [pair.first, pair.second], |[v1, v2]| {
        let ne = if let Ok(v1) = v1
            && let Ok(v2) = v2
        {
            v1 != v2
        } else {
            false
        };
        Val::Bool(Bool::new(ne))
    })
}
