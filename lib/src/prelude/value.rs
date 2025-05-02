use rand::{
    SeedableRng,
    prelude::SmallRng,
};

use crate::{
    Byte,
    Call,
    CodeMode,
    ConstFnCtx,
    Ctx,
    FuncMode,
    Int,
    List,
    Map,
    Number,
    Pair,
    SymbolMode,
    Text,
    Unit,
    Val,
    arbitrary::Arbitrary,
    bit::Bit,
    ctx::{
        main::MainCtx,
        map::CtxMapRef,
        ref1::CtxRef,
    },
    either::Either,
    prelude::{
        Named,
        Prelude,
        PreludeCtx,
        named_const_fn,
        named_free_fn,
        ref_mode,
        ref_pair_mode,
    },
    symbol::Symbol,
    val::{
        BIT,
        BYTE,
        CALL,
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
    pub(crate) type1: Named<FuncVal>,
    pub(crate) equal: Named<FuncVal>,
}

impl Default for ValuePrelude {
    fn default() -> Self {
        ValuePrelude { any: any(), type1: type1(), equal: equal() }
    }
}

impl Prelude for ValuePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.any.put(ctx);
        self.type1.put(ctx);
        self.equal.put(ctx);
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
        Val::Unit(_) => Val::any(rng, DEPTH),
        Val::Symbol(s) => match &*s {
            UNIT => Val::Unit(Unit::any(rng, DEPTH)),
            BIT => Val::Bit(Bit::any(rng, DEPTH)),
            SYMBOL => Val::Symbol(Symbol::any(rng, DEPTH)),
            TEXT => Val::Text(Text::any(rng, DEPTH).into()),
            INT => Val::Int(Int::any(rng, DEPTH).into()),
            NUMBER => Val::Number(Number::any(rng, DEPTH).into()),
            BYTE => Val::Byte(Byte::any(rng, DEPTH).into()),
            PAIR => Val::Pair(Pair::<Val, Val>::any(rng, DEPTH).into()),
            CALL => Val::Call(Call::<Val, Val>::any(rng, DEPTH).into()),
            LIST => Val::List(List::<Val>::any(rng, DEPTH).into()),
            MAP => Val::Map(Map::<Val, Val>::any(rng, DEPTH).into()),
            CTX => Val::Ctx(Ctx::any(rng, DEPTH).into()),
            FUNC => Val::Func(FuncVal::any(rng, DEPTH)),
            EXT => Val::Ext(Arbitrary::any(rng, DEPTH)),
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
    MainCtx::with_ref_lossless(ctx, pair.first, |val| {
        let s = match val {
            Val::Unit(_) => UNIT,
            Val::Bit(_) => BIT,
            Val::Symbol(_) => SYMBOL,
            Val::Text(_) => TEXT,
            Val::Int(_) => INT,
            Val::Number(_) => NUMBER,
            Val::Byte(_) => BYTE,
            Val::Pair(_) => PAIR,
            Val::Call(_) => CALL,
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
    let left = MainCtx::ref_or_val(pair.first);
    let right = MainCtx::ref_or_val(pair.second);
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
