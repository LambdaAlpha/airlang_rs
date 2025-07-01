use std::cell::OnceCell;
use std::rc::Rc;

use self::bit::BitPrelude;
use self::byte::BytePrelude;
use self::call::CallPrelude;
use self::ctrl::CtrlPrelude;
use self::ctx::CtxPrelude;
use self::func::FuncPrelude;
use self::int::IntPrelude;
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
use self::text::TextPrelude;
use self::unit::UnitPrelude;
use self::value::ValuePrelude;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxMap;
use crate::semantics::ctx::CtxValue;
use crate::semantics::func::ConstCellFnVal;
use crate::semantics::func::ConstCellPrimFunc;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::ConstStaticPrimFunc;
use crate::semantics::func::FreeCellFnVal;
use crate::semantics::func::FreeCellPrimFunc;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FreeStaticPrimFunc;
use crate::semantics::func::MutCellFnVal;
use crate::semantics::func::MutCellPrimFunc;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::MutStaticPrimFunc;
use crate::semantics::val::ConstCellPrimFuncVal;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeCellPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutCellPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Map;
use crate::type_::Symbol;

thread_local!(pub(crate) static PRELUDE: OnceCell<Box<dyn Prelude>> = OnceCell::new());

pub trait Prelude {
    fn put(&self, ctx: &mut dyn PreludeCtx);
}

pub trait PreludeCtx {
    fn put(&mut self, name: Symbol, val: Val);
}

pub(crate) fn set_prelude(prelude: Box<dyn Prelude>) {
    PRELUDE.with(|p| {
        let _ = p.set(prelude);
    });
}

pub fn initial_ctx() -> Ctx {
    let mut variables: Map<Symbol, CtxValue> = Map::default();
    put_preludes(&mut variables);
    let variables = CtxMap::new(variables);
    Ctx::new(variables)
}

pub(crate) fn put_preludes(ctx: &mut dyn PreludeCtx) {
    PRELUDE.with(|prelude| {
        let Some(prelude) = prelude.get() else {
            return;
        };
        prelude.put(ctx);
    });
}

impl PreludeCtx for Map<Symbol, CtxValue> {
    fn put(&mut self, name: Symbol, val: Val) {
        let v = self.insert(name, CtxValue::new(val, Contract::default()));
        assert!(v.is_none(), "names of preludes should be unique");
    }
}

impl PreludeCtx for Map<Symbol, Val> {
    fn put(&mut self, name: Symbol, val: Val) {
        let v = self.insert(name, val);
        assert!(v.is_none(), "names of preludes should be unique");
    }
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
    pub call: CallPrelude,
    pub list: ListPrelude,
    pub map: MapPrelude,
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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.unit.put(ctx);
        self.bool.put(ctx);
        self.symbol.put(ctx);
        self.text.put(ctx);
        self.int.put(ctx);
        self.number.put(ctx);
        self.byte.put(ctx);
        self.pair.put(ctx);
        self.call.put(ctx);
        self.list.put(ctx);
        self.map.put(ctx);
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

pub(crate) trait Named {
    fn name(&self) -> Symbol;
}

impl<T: Named + Clone + Into<FuncVal>> Prelude for T {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        ctx.put(self.name(), Val::Func(self.clone().into()));
    }
}

impl Named for FreeCellPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for FreeStaticPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for ConstCellPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for ConstStaticPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for MutCellPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

impl Named for MutStaticPrimFuncVal {
    fn name(&self) -> Symbol {
        self.id.clone()
    }
}

pub struct FreeFn<F> {
    pub id: &'static str,
    pub f: F,
    pub mode: FuncMode,
}

pub struct DynFn<F> {
    pub id: &'static str,
    pub f: F,
    pub mode: FuncMode,
    pub ctx_explicit: bool,
}

impl<F: FreeCellFnVal + 'static> FreeFn<F> {
    pub fn free_cell(self) -> FreeCellPrimFuncVal {
        let func = FreeCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            setup: Some(self.mode.into_setup()),
        };
        FreeCellPrimFuncVal::from(func)
    }
}

impl<F: FreeStaticFn<Val, Val> + 'static> FreeFn<F> {
    pub fn free_static(self) -> FreeStaticPrimFuncVal {
        let func = FreeStaticPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Some(self.mode.into_setup()),
        };
        FreeStaticPrimFuncVal::from(func)
    }
}

impl<F: ConstCellFnVal + 'static> DynFn<F> {
    pub fn const_cell(self) -> ConstCellPrimFuncVal {
        let func = ConstCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            setup: Some(self.mode.into_setup()),
            ctx_explicit: self.ctx_explicit,
        };
        ConstCellPrimFuncVal::from(func)
    }
}

impl<F: ConstStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    pub fn const_static(self) -> ConstStaticPrimFuncVal {
        let func = ConstStaticPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Some(self.mode.into_setup()),
            ctx_explicit: self.ctx_explicit,
        };
        ConstStaticPrimFuncVal::from(func)
    }
}

impl<F: MutCellFnVal + 'static> DynFn<F> {
    pub fn mut_cell(self) -> MutCellPrimFuncVal {
        let func = MutCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            setup: Some(self.mode.into_setup()),
            ctx_explicit: self.ctx_explicit,
        };
        MutCellPrimFuncVal::from(func)
    }
}

impl<F: MutStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    pub fn mut_static(self) -> MutStaticPrimFuncVal {
        let func = MutStaticPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Some(self.mode.into_setup()),
            ctx_explicit: self.ctx_explicit,
        };
        MutStaticPrimFuncVal::from(func)
    }
}

fn ctx_put_val<V: Clone + Into<Val>>(ctx: &mut dyn PreludeCtx, name: &'static str, val: &V) {
    let name = Symbol::from_str_unchecked(name);
    let val = val.clone().into();
    ctx.put(name, val);
}

fn ctx_put_func<V: Clone + Into<FuncVal>>(ctx: &mut dyn PreludeCtx, name: &'static str, val: &V) {
    let name = Symbol::from_str_unchecked(name);
    let func = val.clone().into();
    ctx.put(name, Val::Func(func));
}

pub struct FreeStaticImpl<I, O> {
    pub free: fn(I) -> O,
}

impl<I, O> FreeStaticFn<I, O> for FreeStaticImpl<I, O> {
    fn free_static_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<I, O> FreeStaticImpl<I, O> {
    pub fn new(free: fn(I) -> O) -> Self {
        Self { free }
    }

    pub fn default(_input: I) -> O
    where O: Default {
        O::default()
    }
}

pub struct ConstStaticImpl<Ctx, I, O> {
    pub free: fn(I) -> O,
    pub const_: fn(ConstRef<Ctx>, I) -> O,
}

impl<Ctx, I, O> FreeStaticFn<I, O> for ConstStaticImpl<Ctx, I, O> {
    fn free_static_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<Ctx, I, O> ConstStaticFn<Ctx, I, O> for ConstStaticImpl<Ctx, I, O> {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const_)(ctx, input)
    }
}

impl<Ctx, I, O> ConstStaticImpl<Ctx, I, O> {
    pub fn new(free: fn(I) -> O, const_: fn(ConstRef<Ctx>, I) -> O) -> Self {
        Self { free, const_ }
    }

    pub fn default(_ctx: ConstRef<Ctx>, _input: I) -> O
    where O: Default {
        O::default()
    }
}

pub struct MutStaticImpl<Ctx, I, O> {
    pub free: fn(I) -> O,
    pub const_: fn(ConstRef<Ctx>, I) -> O,
    pub mut_: fn(&mut Ctx, I) -> O,
}

impl<Ctx, I, O> FreeStaticFn<I, O> for MutStaticImpl<Ctx, I, O> {
    fn free_static_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<Ctx, I, O> ConstStaticFn<Ctx, I, O> for MutStaticImpl<Ctx, I, O> {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (self.const_)(ctx, input)
    }
}

impl<Ctx, I, O> MutStaticFn<Ctx, I, O> for MutStaticImpl<Ctx, I, O> {
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        (self.mut_)(ctx, input)
    }
}

impl<Ctx, I, O> MutStaticImpl<Ctx, I, O> {
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

pub fn free_impl(func: fn(Val) -> Val) -> FreeStaticImpl<Val, Val> {
    FreeStaticImpl::new(func)
}

pub fn const_impl(func: fn(ConstRef<Val>, Val) -> Val) -> ConstStaticImpl<Val, Val, Val> {
    ConstStaticImpl::new(FreeStaticImpl::default, func)
}

pub fn mut_impl(func: fn(&mut Val, Val) -> Val) -> MutStaticImpl<Val, Val, Val> {
    MutStaticImpl::new(FreeStaticImpl::default, ConstStaticImpl::default, func)
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

pub mod call;

pub mod list;

pub mod map;

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
