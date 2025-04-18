use rand::{
    SeedableRng,
    prelude::SmallRng,
};

use crate::{
    CodeMode,
    ConstFnCtx,
    Ctx,
    FuncMode,
    Map,
    Pair,
    SymbolMode,
    Val,
    arbitrary::{
        any_abstract,
        any_bit,
        any_byte,
        any_call,
        any_change,
        any_ctx,
        any_either,
        any_equiv,
        any_extension,
        any_func,
        any_generate,
        any_int,
        any_inverse,
        any_list,
        any_map,
        any_number,
        any_pair,
        any_reify,
        any_symbol,
        any_text,
        any_unit,
        any_val,
    },
    bit::Bit,
    ctx::{
        default::DefaultCtx,
        map::{
            CtxMapRef,
            CtxValue,
        },
        ref1::CtxRef,
    },
    either::Either,
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        ref_mode,
        ref_pair_mode,
    },
    symbol::Symbol,
    val::{
        ABSTRACT,
        BIT,
        BYTE,
        CALL,
        CHANGE,
        CTX,
        EITHER,
        EQUIV,
        EXT,
        FUNC,
        GENERATE,
        INT,
        INVERSE,
        LIST,
        MAP,
        NUMBER,
        PAIR,
        REIFY,
        SYMBOL,
        TEXT,
        UNIT,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct ValuePrelude {
    pub(crate) any: Named<FuncVal>,
    pub(crate) type1: Named<FuncVal>,
    pub(crate) equal: Named<FuncVal>,
}

impl Default for ValuePrelude {
    fn default() -> Self {
        ValuePrelude { any: any(), type1: type1(), equal: equal() }
    }
}

impl Prelude for ValuePrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.any.put(m);
        self.type1.put(m);
        self.equal.put(m);
    }
}

fn any() -> Named<FuncVal> {
    let id = "any";
    let f = fn_any;
    let call = FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal);
    let mode = FuncMode { call };
    named_free_fn(id, f, mode)
}

fn fn_any(input: Val) -> Val {
    const DEPTH: usize = 0;
    let mut rng = SmallRng::from_os_rng();
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
            EITHER => Val::Either(any_either(rng, DEPTH).into()),
            CHANGE => Val::Change(any_change(rng, DEPTH).into()),
            CALL => Val::Call(any_call(rng, DEPTH).into()),
            REIFY => Val::Reify(any_reify(rng, DEPTH).into()),
            EQUIV => Val::Equiv(any_equiv(rng, DEPTH).into()),
            INVERSE => Val::Inverse(any_inverse(rng, DEPTH).into()),
            GENERATE => Val::Generate(any_generate(rng, DEPTH).into()),
            ABSTRACT => Val::Abstract(any_abstract(rng, DEPTH).into()),
            LIST => Val::List(any_list(rng, DEPTH).into()),
            MAP => Val::Map(any_map(rng, DEPTH).into()),
            CTX => Val::Ctx(any_ctx(rng, DEPTH).into()),
            FUNC => Val::Func(any_func(rng, DEPTH)),
            EXT => Val::Ext(any_extension(rng, DEPTH)),
            _ => Val::default(),
        },
        _ => Val::default(),
    }
}

fn type1() -> Named<FuncVal> {
    let id = "type";
    let f = fn_type1;
    let call = ref_pair_mode();
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_type1(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let s = match val {
            Val::Unit(_) => UNIT,
            Val::Bit(_) => BIT,
            Val::Symbol(_) => SYMBOL,
            Val::Text(_) => TEXT,
            Val::Int(_) => INT,
            Val::Number(_) => NUMBER,
            Val::Byte(_) => BYTE,
            Val::Pair(_) => PAIR,
            Val::Either(_) => EITHER,
            Val::Change(_) => CHANGE,
            Val::Call(_) => CALL,
            Val::Reify(_) => REIFY,
            Val::Equiv(_) => EQUIV,
            Val::Inverse(_) => INVERSE,
            Val::Generate(_) => GENERATE,
            Val::Abstract(_) => ABSTRACT,
            Val::List(_) => LIST,
            Val::Map(_) => MAP,
            Val::Ctx(_) => CTX,
            Val::Func(_) => FUNC,
            Val::Ext(_) => EXT,
        };
        Val::Symbol(Symbol::from_str(s))
    })
}

fn equal() -> Named<FuncVal> {
    let id = "==";
    let f = fn_equal;
    let call = FuncMode::pair_mode(ref_mode(), ref_mode());
    let mode = FuncMode { call };
    named_const_fn(id, f, mode)
}

fn fn_equal(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let left = DefaultCtx::ref_or_val(pair.first);
    let right = DefaultCtx::ref_or_val(pair.second);
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

fn get_by_ref<T, F>(ctx: Option<&Ctx>, v: Either<Symbol, Val>, f: F) -> T
where F: FnOnce(Option<&Val>) -> T {
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
