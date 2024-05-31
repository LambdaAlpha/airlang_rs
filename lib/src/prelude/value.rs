use rand::{
    prelude::SmallRng,
    SeedableRng,
};

use crate::{
    arbitrary::{
        any_answer,
        any_ask,
        any_assert,
        any_bool,
        any_bytes,
        any_call,
        any_ctx,
        any_extension,
        any_func,
        any_int,
        any_list,
        any_map,
        any_number,
        any_pair,
        any_string,
        any_symbol,
        any_unit,
        any_val,
    },
    bool::Bool,
    ctx::{
        constant::CtxForConstFn,
        ref1::CtxRef,
        CtxMap,
        DefaultCtx,
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
        ASK,
        ASSERT,
        BOOL,
        BYTES,
        CALL,
        CTX,
        EXT,
        FUNC,
        INT,
        LIST,
        MAP,
        NUMBER,
        PAIR,
        STRING,
        SYMBOL,
        UNIT,
    },
    Ctx,
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
    let input_mode = Mode::Predefined(Transform::Lazy);
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
            INT => Val::Int(any_int(rng).into()),
            NUMBER => Val::Number(any_number(rng).into()),
            BYTES => Val::Bytes(any_bytes(rng).into()),
            SYMBOL => Val::Symbol(any_symbol(rng)),
            STRING => Val::String(any_string(rng).into()),
            PAIR => Val::Pair(any_pair(rng, DEPTH).into()),
            CALL => Val::Call(any_call(rng, DEPTH).into()),
            ASK => Val::Ask(any_ask(rng, DEPTH).into()),
            LIST => Val::List(any_list(rng, DEPTH).into()),
            MAP => Val::Map(any_map(rng, DEPTH).into()),
            CTX => Val::Ctx(any_ctx(rng, DEPTH).into()),
            FUNC => Val::Func(FuncVal::new(any_func(rng, DEPTH))),
            ASSERT => Val::Assert(any_assert(rng, DEPTH).into()),
            ANSWER => Val::Answer(any_answer(rng, DEPTH).into()),
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
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let s = match val {
            Val::Unit(_) => UNIT,
            Val::Bool(_) => BOOL,
            Val::Int(_) => INT,
            Val::Number(_) => NUMBER,
            Val::Bytes(_) => BYTES,
            Val::Symbol(_) => SYMBOL,
            Val::String(_) => STRING,
            Val::Pair(_) => PAIR,
            Val::Call(_) => CALL,
            Val::Ask(_) => ASK,
            Val::List(_) => LIST,
            Val::Map(_) => MAP,
            Val::Func(_) => FUNC,
            Val::Ctx(_) => CTX,
            Val::Assert(_) => ASSERT,
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
    let ctx = ctx.borrow();
    let Some(v1) = get_by_ref(ctx, &pair.first) else {
        return Val::default();
    };
    let Some(v2) = get_by_ref(ctx, &pair.second) else {
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
    let ctx = ctx.borrow();
    let Some(v1) = get_by_ref(ctx, &pair.first) else {
        return Val::default();
    };
    let Some(v2) = get_by_ref(ctx, &pair.second) else {
        return Val::default();
    };
    Val::Bool(Bool::new(*v1 != *v2))
}

fn get_by_ref<'a>(ctx: Option<&'a Ctx>, v: &'a Val) -> Option<&'a Val> {
    match v {
        Val::Symbol(v) => {
            let ctx = ctx?;
            if let Ok(v1) = ctx.get_ref(v.clone()) {
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
