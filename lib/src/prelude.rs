use std::rc::Rc;

use self::bit::BitPrelude;
use self::byte::BytePrelude;
use self::ctrl::CtrlPrelude;
use self::ctx::CtxPrelude;
use self::func::FuncPrelude;
use self::int::IntPrelude;
use self::link::LinkPrelude;
use self::list::ListPrelude;
use self::map::MapPrelude;
use self::meta::MetaPrelude;
use self::mode::FuncMode;
use self::mode::ModePrelude;
use self::number::NumberPrelude;
use self::pair::PairPrelude;
use self::solve::SolvePrelude;
use self::symbol::SymbolPrelude;
use self::syntax::SyntaxPrelude;
use self::task::TaskPrelude;
use self::text::TextPrelude;
use self::unit::UnitPrelude;
use self::value::ValuePrelude;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::ConstPrimFunc;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::func::MutFn;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Symbol;

pub trait Prelude {
    fn put(self, ctx: &mut Ctx);
}

#[derive(Default, Clone)]
pub struct CorePrelude {
    pub unit: UnitPrelude,
    pub bool: BitPrelude,
    pub symbol: SymbolPrelude,
    pub text: TextPrelude,
    pub int: IntPrelude,
    pub number: NumberPrelude,
    pub byte: BytePrelude,
    pub pair: PairPrelude,
    pub task: TaskPrelude,
    pub list: ListPrelude,
    pub map: MapPrelude,
    pub link: LinkPrelude,
    pub ctx: CtxPrelude,
    pub func: FuncPrelude,
    pub solve: SolvePrelude,
    pub meta: MetaPrelude,
    pub syntax: SyntaxPrelude,
    pub value: ValuePrelude,
    pub ctrl: CtrlPrelude,
    pub mode: ModePrelude,
}

impl Prelude for CorePrelude {
    fn put(self, ctx: &mut Ctx) {
        self.unit.put(ctx);
        self.bool.put(ctx);
        self.symbol.put(ctx);
        self.text.put(ctx);
        self.int.put(ctx);
        self.number.put(ctx);
        self.byte.put(ctx);
        self.pair.put(ctx);
        self.task.put(ctx);
        self.list.put(ctx);
        self.map.put(ctx);
        self.link.put(ctx);
        self.ctx.put(ctx);
        self.func.put(ctx);
        self.solve.put(ctx);
        self.meta.put(ctx);
        self.syntax.put(ctx);
        self.value.put(ctx);
        self.ctrl.put(ctx);
        self.mode.put(ctx);
    }
}

impl<T: Prelude> From<T> for Ctx {
    fn from(value: T) -> Self {
        let mut ctx = Ctx::default();
        value.put(&mut ctx);
        ctx
    }
}

pub(crate) trait Named {
    fn name(&self) -> Symbol;
}

impl<T: Named + Clone + Into<FuncVal>> Prelude for T {
    fn put(self, ctx: &mut Ctx) {
        let v = ctx.put(self.name(), Val::Func(self.into()), Contract::None);
        assert!(matches!(v, Ok(None)), "names of preludes should be unique");
    }
}

impl Named for FreePrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for ConstPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for MutPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

pub struct FreePrimFn<F> {
    pub id: &'static str,
    pub f: F,
    pub mode: FuncMode,
}

pub struct DynPrimFn<F> {
    pub id: &'static str,
    pub f: F,
    pub mode: FuncMode,
}

impl<F: FreeFn<Val, Val> + 'static> FreePrimFn<F> {
    pub fn free(self) -> FreePrimFuncVal {
        let func = FreePrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: self.mode.into_setup(),
        };
        FreePrimFuncVal::from(func)
    }
}

impl<F: ConstFn<Val, Val, Val> + 'static> DynPrimFn<F> {
    pub fn const_(self) -> ConstPrimFuncVal {
        let func = ConstPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: self.mode.into_setup(),
        };
        ConstPrimFuncVal::from(func)
    }
}

impl<F: MutFn<Val, Val, Val> + 'static> DynPrimFn<F> {
    pub fn mut_(self) -> MutPrimFuncVal {
        let func = MutPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: self.mode.into_setup(),
        };
        MutPrimFuncVal::from(func)
    }
}

fn ctx_put_val<V: Into<Val>>(ctx: &mut Ctx, name: &'static str, val: V) {
    let name = Symbol::from_str_unchecked(name);
    let v = ctx.put(name, val.into(), Contract::None);
    assert!(matches!(v, Ok(None)), "names of preludes should be unique");
}

fn ctx_put_func<V: Into<FuncVal>>(ctx: &mut Ctx, name: &'static str, val: V) {
    let name = Symbol::from_str_unchecked(name);
    let v = ctx.put(name, Val::Func(val.into()), Contract::None);
    assert!(matches!(v, Ok(None)), "names of preludes should be unique");
}

pub struct FreeImpl<I, O> {
    pub free: fn(I) -> O,
}

impl<I, O> FreeFn<I, O> for FreeImpl<I, O> {
    fn free_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<I, O> FreeImpl<I, O> {
    pub fn new(free: fn(I) -> O) -> Self {
        Self { free }
    }

    pub fn default(_input: I) -> O
    where O: Default {
        O::default()
    }
}

pub struct ConstImpl<Ctx, I, O> {
    pub free: fn(I) -> O,
    pub const_: fn(ConstRef<Ctx>, I) -> O,
}

impl<Ctx, I, O> FreeFn<I, O> for ConstImpl<Ctx, I, O> {
    fn free_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<Ctx, I, O> ConstFn<Ctx, I, O> for ConstImpl<Ctx, I, O> {
    fn const_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const_)(ctx, input)
    }
}

impl<Ctx, I, O> ConstImpl<Ctx, I, O> {
    pub fn new(free: fn(I) -> O, const_: fn(ConstRef<Ctx>, I) -> O) -> Self {
        Self { free, const_ }
    }

    pub fn default(_ctx: ConstRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}

pub struct MutImpl<Ctx, I, O> {
    pub free: fn(I) -> O,
    pub const_: fn(ConstRef<Ctx>, I) -> O,
    pub mut_: fn(&mut Ctx, I) -> O,
}

impl<Ctx, I, O> FreeFn<I, O> for MutImpl<Ctx, I, O> {
    fn free_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<Ctx, I, O> ConstFn<Ctx, I, O> for MutImpl<Ctx, I, O> {
    fn const_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const_)(ctx, input)
    }
}

impl<Ctx, I, O> MutFn<Ctx, I, O> for MutImpl<Ctx, I, O> {
    fn mut_call(&self, ctx: &mut Ctx, input: I) -> O {
        (self.mut_)(ctx, input)
    }
}

impl<Ctx, I, O> MutImpl<Ctx, I, O> {
    pub fn new(
        free: fn(I) -> O, const_: fn(ConstRef<Ctx>, I) -> O, mut_: fn(&mut Ctx, I) -> O,
    ) -> Self {
        Self { free, const_, mut_ }
    }

    pub fn default(_ctx: DynRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}

pub fn free_impl(func: fn(Val) -> Val) -> FreeImpl<Val, Val> {
    FreeImpl::new(func)
}

pub fn const_impl(func: fn(ConstRef<Val>, Val) -> Val) -> ConstImpl<Val, Val, Val> {
    ConstImpl::new(FreeImpl::default, func)
}

pub fn mut_impl(func: fn(&mut Val, Val) -> Val) -> MutImpl<Val, Val, Val> {
    MutImpl::new(FreeImpl::default, ConstImpl::default, func)
}

pub mod setup;

// -----

pub mod unit;

pub mod bit;

pub mod symbol;

pub mod text;

pub mod int;

pub mod number;

pub mod byte;

pub mod pair;

pub mod task;

pub mod list;

pub mod map;

pub mod link;

pub mod ctx;

pub mod func;

// -----

pub mod solve;

pub mod meta;

pub mod syntax;

pub mod value;

pub mod ctrl;

pub mod mode;

// -----

mod utils;
