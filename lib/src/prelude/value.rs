use log::error;

pub use self::arbitrary::Arbitrary;
pub use self::arbitrary::ArbitraryVal;

_____!();

use rand::SeedableRng;
use rand::prelude::SmallRng;

use self::arbitrary::arbitrary_ext_type;
use self::arbitrary::set_arbitrary_val;
use super::DynFn;
use super::FreeFn;
use super::FuncMode;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::ctx::ref_::RefCtx;
use super::free_impl;
use super::mode::CodeMode;
use super::mode::SymbolMode;
use super::setup::default_dyn_mode;
use super::setup::dyn_mode;
use super::setup::free_mode;
use super::setup::ref_mode;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::BIT;
use crate::semantics::val::BYTE;
use crate::semantics::val::CTX;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FUNC;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::INT;
use crate::semantics::val::LIST;
use crate::semantics::val::MAP;
use crate::semantics::val::NUMBER;
use crate::semantics::val::PAIR;
use crate::semantics::val::SYMBOL;
use crate::semantics::val::TASK;
use crate::semantics::val::TEXT;
use crate::semantics::val::UNIT;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::ConstRef;
use crate::type_::Either;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;
use crate::type_::Text;
use crate::type_::Unit;

pub fn init_arbitrary(arbitrary_val: Box<dyn ArbitraryVal>) {
    set_arbitrary_val(arbitrary_val);
}

#[derive(Clone)]
pub struct ValuePrelude {
    pub any: FreeStaticPrimFuncVal,
    pub type_: ConstStaticPrimFuncVal,
    pub equal: ConstStaticPrimFuncVal,
}

impl Default for ValuePrelude {
    fn default() -> Self {
        ValuePrelude { any: any(), type_: type_(), equal: equal() }
    }
}

impl Prelude for ValuePrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.any.put(ctx);
        self.type_.put(ctx);
        self.equal.put(ctx);
    }
}

// todo design
pub fn any() -> FreeStaticPrimFuncVal {
    FreeFn {
        id: "any",
        f: free_impl(fn_any),
        mode: free_mode(FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form)),
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
            TASK => Val::Task(Task::<Val, Val, Val>::any(rng, DEPTH).into()),
            LIST => Val::List(List::<Val>::any(rng, DEPTH).into()),
            MAP => Val::Map(Map::<Val, Val>::any(rng, DEPTH).into()),
            CTX => Val::Ctx(Ctx::any(rng, DEPTH).into()),
            FUNC => Val::Func(FuncVal::any(rng, DEPTH)),
            _ => arbitrary_ext_type(s),
        },
        v => {
            error!("input {v:?} should be a symbol or a unit");
            Val::default()
        }
    }
}

pub fn type_() -> ConstStaticPrimFuncVal {
    DynFn { id: "type", f: const_impl(fn_type), mode: default_dyn_mode() }.const_static()
}

fn fn_type(ctx: ConstRef<Val>, _input: Val) -> Val {
    let s = match &*ctx {
        Val::Unit(_) => UNIT,
        Val::Bit(_) => BIT,
        Val::Symbol(_) => SYMBOL,
        Val::Text(_) => TEXT,
        Val::Int(_) => INT,
        Val::Number(_) => NUMBER,
        Val::Byte(_) => BYTE,
        Val::Pair(_) => PAIR,
        Val::Task(_) => TASK,
        Val::List(_) => LIST,
        Val::Map(_) => MAP,
        Val::Ctx(_) => CTX,
        Val::Func(_) => FUNC,
        Val::Ext(ext) => return Val::Symbol(ext.type_name()),
    };
    Val::Symbol(Symbol::from_str_unchecked(s))
}

// todo design mode and ref
pub fn equal() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "==",
        f: const_impl(fn_equal),
        mode: dyn_mode(FuncMode::pair_mode(Map::default(), ref_mode(), ref_mode())),
    }
    .const_static()
}

fn fn_equal(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Ctx(ctx) = &*ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let left = RefCtx::ref_or_val(pair.first);
    let right = RefCtx::ref_or_val(pair.second);
    get_by_ref(ctx, left, |v1| {
        let Some(v1) = v1 else {
            error!("input.first should exist");
            return Val::default();
        };
        get_by_ref(ctx, right, |v2| {
            let Some(v2) = v2 else {
                error!("input.second should exist");
                return Val::default();
            };
            Val::Bit(Bit::from(*v1 == *v2))
        })
    })
}

fn get_by_ref<T, F>(ctx: &Ctx, v: Either<Symbol, Val>, f: F) -> T
where F: FnOnce(Option<&Val>) -> T {
    match v {
        Either::This(s) => {
            let Ok(val) = ctx.get_ref(s.clone()) else {
                error!("variable {s:?} should exist");
                return f(None);
            };
            f(Some(val))
        }
        Either::That(val) => f(Some(&val)),
    }
}

mod arbitrary;
