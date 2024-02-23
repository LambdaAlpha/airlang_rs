use rand::{
    prelude::SmallRng,
    SeedableRng,
};

use crate::{
    bool::Bool,
    ctx::{
        CtxTrait,
        DefaultCtx,
        NameMap,
    },
    ctx_access::constant::CtxForConstFn,
    eval_mode::EvalMode,
    io_mode::IoMode,
    nondeterministic::{
        any_answer,
        any_bool,
        any_bytes,
        any_call,
        any_ctx,
        any_extension,
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
        default_mode,
        named_const_fn,
        named_free_fn,
        pair_mode,
        symbol_value_mode,
        Named,
        Prelude,
    },
    symbol::Symbol,
    val::func::FuncVal,
    Val,
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
const ANSWER: &str = "answer";
const EXT: &str = "extension";

#[derive(Clone)]
pub(crate) struct ValuePrelude {
    pub(crate) any: Named<FuncVal>,
    pub(crate) type_of: Named<FuncVal>,
    pub(crate) equal: Named<FuncVal>,
    pub(crate) equal_val: Named<FuncVal>,
    pub(crate) equal_ref_val: Named<FuncVal>,
    pub(crate) equal_val_ref: Named<FuncVal>,
    pub(crate) not_equal: Named<FuncVal>,
    pub(crate) not_equal_val: Named<FuncVal>,
    pub(crate) not_equal_ref_val: Named<FuncVal>,
    pub(crate) not_equal_val_ref: Named<FuncVal>,
}

impl Default for ValuePrelude {
    fn default() -> Self {
        ValuePrelude {
            any: any(),
            type_of: type_of(),
            equal: equal(),
            equal_val: equal_val(),
            equal_ref_val: equal_ref_val(),
            equal_val_ref: equal_val_ref(),
            not_equal: not_equal(),
            not_equal_val: not_equal_val(),
            not_equal_ref_val: not_equal_ref_val(),
            not_equal_val_ref: not_equal_val_ref(),
        }
    }
}

impl Prelude for ValuePrelude {
    fn put(&self, m: &mut NameMap) {
        self.any.put(m);
        self.type_of.put(m);
        self.equal.put(m);
        self.equal_val.put(m);
        self.equal_ref_val.put(m);
        self.equal_val_ref.put(m);
        self.not_equal.put(m);
        self.not_equal_val.put(m);
        self.not_equal_ref_val.put(m);
        self.not_equal_val_ref.put(m);
    }
}

fn any() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Lazy);
    let output_mode = default_mode();
    named_free_fn("any", input_mode, output_mode, fn_any)
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
            ANSWER => Val::Answer(any_answer(rng, DEPTH)),
            EXT => Val::Ext(any_extension(rng, DEPTH)),
            _ => Val::default(),
        },
        _ => Val::default(),
    }
}

fn type_of() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = symbol_value_mode();
    named_const_fn("type_of", input_mode, output_mode, fn_type_of)
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
            Val::Answer(_) => ANSWER,
            Val::Ext(_) => EXT,
        };
        Val::Symbol(Symbol::from_str(s))
    })
}

fn equal() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_value_mode(), symbol_value_mode());
    let output_mode = default_mode();
    named_const_fn("===", input_mode, output_mode, fn_equal)
}

fn fn_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(v1) = pair.first else {
        return Val::default();
    };
    let Val::Symbol(v2) = pair.second else {
        return Val::default();
    };
    let v1 = ctx.get_const_ref(&v1);
    let Ok(v1) = v1 else {
        return Val::default();
    };
    let v2 = ctx.get_const_ref(&v2);
    let Ok(v2) = v2 else {
        return Val::default();
    };
    Val::Bool(Bool::new(*v1 == *v2))
}

fn equal_val() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn("-=-", input_mode, output_mode, fn_equal_val)
}

fn fn_equal_val(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first == pair.second))
}

fn equal_ref_val() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_value_mode(), default_mode());
    let output_mode = default_mode();
    named_const_fn("==-", input_mode, output_mode, fn_equal_ref_val)
}

fn fn_equal_ref_val(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(v1) = pair.first else {
        return Val::default();
    };
    let Ok(v1) = ctx.get_const_ref(&v1) else {
        return Val::default();
    };
    Val::Bool(Bool::new(*v1 == pair.second))
}

fn equal_val_ref() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), symbol_value_mode());
    let output_mode = default_mode();
    named_const_fn("-==", input_mode, output_mode, fn_equal_val_ref)
}

fn fn_equal_val_ref(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(v2) = pair.second else {
        return Val::default();
    };
    let Ok(v2) = ctx.get_const_ref(&v2) else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first == *v2))
}

fn not_equal() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_value_mode(), symbol_value_mode());
    let output_mode = default_mode();
    named_const_fn("=/=", input_mode, output_mode, fn_not_equal)
}

fn fn_not_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(v1) = pair.first else {
        return Val::default();
    };
    let Val::Symbol(v2) = pair.second else {
        return Val::default();
    };
    let v1 = ctx.get_const_ref(&v1);
    let Ok(v1) = v1 else {
        return Val::default();
    };
    let v2 = ctx.get_const_ref(&v2);
    let Ok(v2) = v2 else {
        return Val::default();
    };
    Val::Bool(Bool::new(*v1 != *v2))
}

fn not_equal_val() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), default_mode());
    let output_mode = default_mode();
    named_free_fn("-/-", input_mode, output_mode, fn_not_equal_val)
}

fn fn_not_equal_val(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first != pair.second))
}

fn not_equal_ref_val() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_value_mode(), default_mode());
    let output_mode = default_mode();
    named_const_fn("=/-", input_mode, output_mode, fn_not_equal_ref_val)
}

fn fn_not_equal_ref_val(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(v1) = pair.first else {
        return Val::default();
    };
    let Ok(v1) = ctx.get_const_ref(&v1) else {
        return Val::default();
    };
    Val::Bool(Bool::new(*v1 != pair.second))
}

fn not_equal_val_ref() -> Named<FuncVal> {
    let input_mode = pair_mode(default_mode(), symbol_value_mode());
    let output_mode = default_mode();
    named_const_fn("-/=", input_mode, output_mode, fn_not_equal_val_ref)
}

fn fn_not_equal_val_ref(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(v2) = pair.second else {
        return Val::default();
    };
    let Ok(v2) = ctx.get_const_ref(&v2) else {
        return Val::default();
    };
    Val::Bool(Bool::new(pair.first != *v2))
}
