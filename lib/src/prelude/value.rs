use rand::{
    SeedableRng,
    prelude::SmallRng,
};

use crate::{
    Ctx,
    FuncMode,
    Map,
    Mode,
    Val,
    arbitrary::{
        any_abstract,
        any_answer,
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
    core::{
        SYMBOL_ID_PREFIX,
        SYMBOL_REF_PREFIX,
    },
    ctx::{
        CtxValue,
        const1::ConstFnCtx,
        default::DefaultCtx,
        map::CtxMapRef,
        ref1::CtxRef,
    },
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
    },
    symbol::Symbol,
    val::{
        ABSTRACT,
        ANSWER,
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
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    let f = fn_any;
    named_free_fn(id, mode, cacheable, f)
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
            ANSWER => Val::Answer(any_answer(rng, DEPTH).into()),
            EXT => Val::Ext(any_extension(rng, DEPTH)),
            _ => Val::default(),
        },
        _ => Val::default(),
    }
}

fn type_of() -> Named<FuncVal> {
    let id = "type_of";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    let f = fn_type_of;
    named_const_fn(id, mode, cacheable, f)
}

fn fn_type_of(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
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
            Val::Answer(_) => ANSWER,
            Val::Ext(_) => EXT,
        };
        Val::Symbol(Symbol::from_str(s))
    })
}

fn equal() -> Named<FuncVal> {
    let id = "==";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    let f = fn_equal;
    named_const_fn(id, mode, cacheable, f)
}

fn fn_equal(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx = ctx.borrow();
    get_by_ref(ctx, &pair.first, |v1| {
        let Some(v1) = v1 else {
            return Val::default();
        };
        get_by_ref(ctx, &pair.second, |v2| {
            let Some(v2) = v2 else {
                return Val::default();
            };
            Val::Bit(Bit::new(*v1 == *v2))
        })
    })
}

fn not_equal() -> Named<FuncVal> {
    let id = "!=";
    let call = Mode::default();
    let abstract1 = call.clone();
    let ask = Mode::default();
    let mode = FuncMode {
        call,
        abstract1,
        ask,
    };
    let cacheable = true;
    let f = fn_not_equal;
    named_const_fn(id, mode, cacheable, f)
}

fn fn_not_equal(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx = ctx.borrow();
    get_by_ref(ctx, &pair.first, |v1| {
        let Some(v1) = v1 else {
            return Val::default();
        };
        get_by_ref(ctx, &pair.second, |v2| {
            let Some(v2) = v2 else {
                return Val::default();
            };
            Val::Bit(Bit::new(*v1 != *v2))
        })
    })
}

fn get_by_ref<T, F>(ctx: Option<&Ctx>, v: &Val, f: F) -> T
where
    F: FnOnce(Option<&Val>) -> T,
{
    match v {
        Val::Symbol(s) => {
            let prefix = s.chars().next();
            if let Some(SYMBOL_ID_PREFIX) = prefix {
                let s = Symbol::from_str(&s[1..]);
                return f(Some(&Val::Symbol(s)));
            }
            let s = if let Some(SYMBOL_REF_PREFIX) = prefix {
                Symbol::from_str(&s[1..])
            } else {
                s.clone()
            };
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
        val => f(Some(val)),
    }
}
