use rand::SeedableRng;
use rand::prelude::SmallRng;

use crate::Byte;
use crate::Call;
use crate::CodeMode;
use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::Ctx;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::Int;
use crate::List;
use crate::Map;
use crate::Number;
use crate::Pair;
use crate::SymbolMode;
use crate::Text;
use crate::Unit;
use crate::Val;
use crate::bit::Bit;
use crate::ctx::main::MainCtx;
use crate::either::Either;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::ref_mode;
use crate::symbol::Symbol;
use crate::type1::arbitrary::Arbitrary;
use crate::type1::arbitrary::arbitrary_ext_type;
use crate::val::BIT;
use crate::val::BYTE;
use crate::val::CALL;
use crate::val::CTX;
use crate::val::FUNC;
use crate::val::INT;
use crate::val::LIST;
use crate::val::MAP;
use crate::val::NUMBER;
use crate::val::PAIR;
use crate::val::SYMBOL;
use crate::val::TEXT;
use crate::val::UNIT;
use crate::val::func::FuncVal;

#[derive(Clone)]
pub(crate) struct ValuePrelude {
    pub(crate) any: FreeStaticPrimFuncVal,
    pub(crate) type1: ConstStaticPrimFuncVal,
    pub(crate) equal: ConstStaticPrimFuncVal,
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

// todo design
fn any() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "any",
        f: free_impl(fn_any),
        mode: FuncMode {
            forward: FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form),
            reverse: FuncMode::default_mode(),
        },
    }
    .free_static()
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
            _ => arbitrary_ext_type(s),
        },
        _ => Val::default(),
    }
}

fn type1() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "type",
        f: const_impl(fn_type1),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_type1(ctx: ConstRef<Val>, _input: Val) -> Val {
    let s = match &*ctx {
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
        Val::Ext(ext) => return Val::Symbol(ext.type_name()),
    };
    Val::Symbol(Symbol::from_str(s))
}

// todo design mode and ref
fn equal() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "==",
        f: const_impl(fn_equal),
        mode: FuncMode {
            forward: FuncMode::pair_mode(ref_mode(), ref_mode()),
            reverse: FuncMode::default_mode(),
        },
        ctx_explicit: false,
    }
    .const_static()
}

fn fn_equal(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let left = MainCtx::ref_or_val(pair.first);
    let right = MainCtx::ref_or_val(pair.second);
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

fn get_by_ref<T, F>(ctx: &Ctx, v: Either<Symbol, Val>, f: F) -> T
where F: FnOnce(Option<&Val>) -> T {
    match v {
        Either::This(s) => {
            let Ok(val) = ctx.variables().get_ref(s) else {
                return f(None);
            };
            f(Some(val))
        }
        Either::That(val) => f(Some(&val)),
    }
}
