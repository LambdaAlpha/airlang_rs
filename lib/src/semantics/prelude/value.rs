use {
    crate::{
        semantics::{
            ctx::{
                DefaultCtx,
                NameMap,
            },
            ctx_access::constant::CtxForConstFn,
            eval_mode::EvalMode,
            input_mode::InputMode,
            nondeterministic::{
                any_bool,
                any_bytes,
                any_call,
                any_ctx,
                any_float,
                any_func,
                any_int,
                any_list,
                any_map,
                any_pair,
                any_prop,
                any_reverse,
                any_string,
                any_symbol,
                any_unit,
                any_val,
            },
            prelude::{
                named_const_fn,
                named_free_fn,
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
    },
    rand::{
        prelude::SmallRng,
        SeedableRng,
    },
};

const UNIT: &str = "unit";
const BOOL: &str = "bool";
const INT: &str = "int";
const FLOAT: &str = "float";
const BYTES: &str = "bytes";
const SYMBOL: &str = "symbol";
const STRING: &str = "string";
const PAIR: &str = "pair";
const CALL: &str = "call";
const REVERSE: &str = "reverse";
const LIST: &str = "list";
const MAP: &str = "map";
const CTX: &str = "context";
const FUNC: &str = "function";
const PROP: &str = "proposition";

#[derive(Clone)]
pub(crate) struct ValuePrelude {
    pub(crate) any: Named<FuncVal>,
    pub(crate) type_of: Named<FuncVal>,
    pub(crate) equal: Named<FuncVal>,
    pub(crate) not_equal: Named<FuncVal>,
}

impl Default for ValuePrelude {
    fn default() -> Self {
        ValuePrelude {
            any: any(),
            type_of: type_of(),
            equal: equal(),
            not_equal: not_equal(),
        }
    }
}

impl Prelude for ValuePrelude {
    fn put(&self, m: &mut NameMap) {
        self.any.put(m);
        self.type_of.put(m);
        self.equal.put(m);
        self.not_equal.put(m);
    }
}

fn any() -> Named<FuncVal> {
    let input_mode = InputMode::Any(EvalMode::Mix);
    named_free_fn("any", input_mode, fn_any)
}

fn fn_any(input: Val) -> Val {
    const DEPTH: usize = 0;
    let mut rng = SmallRng::from_entropy();
    let rng = &mut rng;
    match input {
        Val::Unit(_) => any_val(rng, DEPTH),
        Val::Symbol(s) => match &*s {
            UNIT => Val::Unit(any_unit(rng)),
            BOOL => Val::Bool(any_bool(rng)),
            INT => Val::Int(any_int(rng)),
            FLOAT => Val::Float(any_float(rng)),
            BYTES => Val::Bytes(any_bytes(rng)),
            SYMBOL => Val::Symbol(any_symbol(rng)),
            STRING => Val::String(any_string(rng)),
            PAIR => Val::Pair(Box::new(any_pair(rng, DEPTH))),
            CALL => Val::Call(Box::new(any_call(rng, DEPTH))),
            REVERSE => Val::Reverse(Box::new(any_reverse(rng, DEPTH))),
            LIST => Val::List(any_list(rng, DEPTH)),
            MAP => Val::Map(any_map(rng, DEPTH)),
            CTX => Val::Ctx(any_ctx(rng, DEPTH)),
            FUNC => Val::Func(any_func(rng, DEPTH)),
            PROP => Val::Prop(any_prop(rng, DEPTH)),
            _ => Val::default(),
        },
        _ => Val::default(),
    }
}

fn type_of() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("type_of", input_mode, fn_type_of)
}

fn fn_type_of(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let s = match val {
            Val::Unit(_) => UNIT,
            Val::Bool(_) => BOOL,
            Val::Int(_) => INT,
            Val::Float(_) => FLOAT,
            Val::Bytes(_) => BYTES,
            Val::Symbol(_) => SYMBOL,
            Val::String(_) => STRING,
            Val::Pair(_) => PAIR,
            Val::Call(_) => CALL,
            Val::Reverse(_) => REVERSE,
            Val::List(_) => LIST,
            Val::Map(_) => MAP,
            Val::Func(_) => FUNC,
            Val::Ctx(_) => CTX,
            Val::Prop(_) => PROP,
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
