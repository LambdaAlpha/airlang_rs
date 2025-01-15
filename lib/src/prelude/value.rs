use rand::{
    SeedableRng,
    prelude::SmallRng,
};

use crate::{
    Ctx,
    FuncMode,
    Map,
    Mode,
    MutFnCtx,
    Pair,
    PrefixMode,
    Val,
    arbitrary::{
        any_abstract,
        any_ask,
        any_bit,
        any_byte,
        any_call,
        any_case_val,
        any_ctx,
        any_extension,
        any_func,
        any_int,
        any_list,
        any_map,
        any_number,
        any_pair,
        any_symbol,
        any_text,
        any_unit,
        any_val,
    },
    bit::Bit,
    ctx::{
        CtxValue,
        default::DefaultCtx,
        map::CtxMapRef,
        ref1::CtxRef,
    },
    prelude::{
        Named,
        Prelude,
        form_mode,
        id_mode,
        named_free_fn,
        named_mut_fn,
        pair_mode,
        symbol_literal_mode,
    },
    symbol::Symbol,
    types::either::Either,
    val::{
        ABSTRACT,
        ASK,
        BIT,
        BYTE,
        CALL,
        CASE,
        CTX,
        EXT,
        FUNC,
        INT,
        LIST,
        MAP,
        NUMBER,
        PAIR,
        SYMBOL,
        TEXT,
        UNIT,
        func::FuncVal,
    },
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.any.put(m);
        self.type_of.put(m);
        self.equal.put(m);
        self.not_equal.put(m);
    }
}

fn any() -> Named<FuncVal> {
    let id = "any";
    let f = fn_any;
    let call = form_mode(PrefixMode::Literal);
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_free_fn(id, f, mode, cacheable)
}

fn fn_any(input: Val) -> Val {
    const DEPTH: usize = 0;
    let mut rng = SmallRng::from_entropy();
    let rng = &mut rng;
    match input {
        Val::Unit(_) => any_val(rng, DEPTH),
        Val::Symbol(s) => match &*s {
            UNIT => Val::Unit(any_unit(rng)),
            BIT => Val::Bit(any_bit(rng)),
            SYMBOL => Val::Symbol(any_symbol(rng)),
            TEXT => Val::Text(any_text(rng).into()),
            INT => Val::Int(any_int(rng).into()),
            NUMBER => Val::Number(any_number(rng).into()),
            BYTE => Val::Byte(any_byte(rng).into()),
            PAIR => Val::Pair(any_pair(rng, DEPTH).into()),
            ABSTRACT => Val::Abstract(any_abstract(rng, DEPTH).into()),
            CALL => Val::Call(any_call(rng, DEPTH).into()),
            ASK => Val::Ask(any_ask(rng, DEPTH).into()),
            LIST => Val::List(any_list(rng, DEPTH).into()),
            MAP => Val::Map(any_map(rng, DEPTH).into()),
            CTX => Val::Ctx(any_ctx(rng, DEPTH).into()),
            FUNC => Val::Func(any_func(rng, DEPTH)),
            CASE => Val::Case(any_case_val(rng, DEPTH)),
            EXT => Val::Ext(any_extension(rng, DEPTH)),
            _ => Val::default(),
        },
        _ => Val::default(),
    }
}

fn type_of() -> Named<FuncVal> {
    let id = "type_of";
    let f = fn_type_of;
    let call = id_mode();
    let abstract1 = call.clone();
    let ask = symbol_literal_mode();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_type_of(ctx: MutFnCtx, input: Val) -> Val {
    DefaultCtx::with_ref_lossless(ctx, input, |val| {
        let s = match val {
            Val::Unit(_) => UNIT,
            Val::Bit(_) => BIT,
            Val::Symbol(_) => SYMBOL,
            Val::Text(_) => TEXT,
            Val::Int(_) => INT,
            Val::Number(_) => NUMBER,
            Val::Byte(_) => BYTE,
            Val::Pair(_) => PAIR,
            Val::Abstract(_) => ABSTRACT,
            Val::Call(_) => CALL,
            Val::Ask(_) => ASK,
            Val::List(_) => LIST,
            Val::Map(_) => MAP,
            Val::Func(_) => FUNC,
            Val::Ctx(_) => CTX,
            Val::Case(_) => CASE,
            Val::Ext(_) => EXT,
        };
        Val::Symbol(Symbol::from_str(s))
    })
}

fn equal() -> Named<FuncVal> {
    let id = "==";
    let f = fn_equal;
    let call = pair_mode(id_mode(), id_mode());
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_equal(mut ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let left = DefaultCtx::ref_or_val(ctx.reborrow(), pair.first);
    let right = DefaultCtx::ref_or_val(ctx.reborrow(), pair.second);
    let ctx = ctx.borrow();
    get_by_ref(ctx, left, |v1| {
        let Some(v1) = v1 else {
            return Val::default();
        };
        get_by_ref(ctx, right, |v2| {
            let Some(v2) = v2 else {
                return Val::default();
            };
            Val::Bit(Bit::new(*v1 == *v2))
        })
    })
}

fn not_equal() -> Named<FuncVal> {
    let id = "!=";
    let f = fn_not_equal;
    let call = pair_mode(id_mode(), id_mode());
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    named_mut_fn(id, f, mode, cacheable)
}

fn fn_not_equal(mut ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let left = DefaultCtx::ref_or_val(ctx.reborrow(), pair.first);
    let right = DefaultCtx::ref_or_val(ctx.reborrow(), pair.second);
    let ctx = ctx.borrow();
    get_by_ref(ctx, left, |v1| {
        let Some(v1) = v1 else {
            return Val::default();
        };
        get_by_ref(ctx, right, |v2| {
            let Some(v2) = v2 else {
                return Val::default();
            };
            Val::Bit(Bit::new(*v1 != *v2))
        })
    })
}

fn get_by_ref<T, F>(ctx: Option<&Ctx>, v: Either<Symbol, Val>, f: F) -> T
where
    F: FnOnce(Option<&Val>) -> T,
{
    match v {
        Either::This(s) => {
            let Some(ctx) = ctx else {
                return f(None);
            };
            let Ok(ctx) = ctx.get_variables() else {
                return f(None);
            };
            let Ok(val) = ctx.get_ref(s) else {
                return f(None);
            };
            f(Some(val))
        }
        Either::That(val) => f(Some(&val)),
    }
}
