use rand::{
    prelude::SmallRng,
    SeedableRng,
};

use crate::{
    bool::Bool,
    ctx::{
        CtxMap,
        CtxTrait,
        DefaultCtx,
    },
    ctx_access::constant::CtxForConstFn,
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
        symbol_id_mode,
        Named,
        Prelude,
    },
    symbol::Symbol,
    transform::Transform,
    val::{
        func::FuncVal,
        ANSWER,
        BOOL,
        BYTES,
        CALL,
        CTX,
        EXT,
        FLOAT,
        FUNC,
        INT,
        LIST,
        MAP,
        PAIR,
        PROP,
        REVERSE,
        STRING,
        SYMBOL,
        UNIT,
    },
    Mode,
    Val,
};

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
    fn put(&self, m: &mut CtxMap) {
        self.any.put(m);
        self.type_of.put(m);
        self.equal.put(m);
        self.not_equal.put(m);
    }
}

fn any() -> Named<FuncVal> {
    let input_mode = Mode::Generic(Transform::Lazy);
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
    let input_mode = symbol_id_mode();
    let output_mode = symbol_id_mode();
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
    let input_mode = pair_mode(symbol_id_mode(), symbol_id_mode());
    let output_mode = default_mode();
    named_const_fn("==", input_mode, output_mode, fn_equal)
}

fn fn_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Some(v1) = get_by_ref(&ctx, &pair.first) else {
        return Val::default();
    };
    let Some(v2) = get_by_ref(&ctx, &pair.second) else {
        return Val::default();
    };
    Val::Bool(Bool::new(*v1 == *v2))
}

fn not_equal() -> Named<FuncVal> {
    let input_mode = pair_mode(symbol_id_mode(), symbol_id_mode());
    let output_mode = default_mode();
    named_const_fn("!=", input_mode, output_mode, fn_not_equal)
}

fn fn_not_equal(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Some(v1) = get_by_ref(&ctx, &pair.first) else {
        return Val::default();
    };
    let Some(v2) = get_by_ref(&ctx, &pair.second) else {
        return Val::default();
    };
    Val::Bool(Bool::new(*v1 != *v2))
}

fn get_by_ref<'a>(ctx: &'a CtxForConstFn<'a>, v: &'a Val) -> Option<&'a Val> {
    match v {
        Val::Symbol(v) => {
            if let Ok(v1) = ctx.get_const_ref(v) {
                Some(v1)
            } else {
                None
            }
        }
        Val::Call(c) => {
            if c.func.is_unit() {
                Some(&c.input)
            } else {
                Some(v)
            }
        }
        v => Some(v),
    }
}
