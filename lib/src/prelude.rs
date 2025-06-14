pub use self::bit::BitPrelude;
pub use self::byte::BytePrelude;
pub use self::call::CallPrelude;
pub use self::ctrl::CtrlPrelude;
pub use self::ctx::CtxPrelude;
pub use self::func::FuncPrelude;
pub use self::int::IntPrelude;
pub use self::list::ListPrelude;
pub use self::map::MapPrelude;
pub use self::meta::MetaPrelude;
pub use self::mode::ModePrelude;
pub use self::number::NumberPrelude;
pub use self::pair::PairPrelude;
pub use self::solve::SolvePrelude;
pub use self::symbol::SymbolPrelude;
pub use self::syntax::SyntaxPrelude;
pub use self::text::TextPrelude;
pub use self::unit::UnitPrelude;
pub use self::value::Arbitrary;
pub use self::value::ArbitraryVal;
pub use self::value::ValuePrelude;

_____!();

use std::cell::RefCell;
use std::rc::Rc;

use self::mode::MODE_PRELUDE;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxMap;
use crate::semantics::ctx::CtxValue;
use crate::semantics::func::ConstCellFnExt;
use crate::semantics::func::ConstCellPrimFunc;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::ConstStaticImpl;
use crate::semantics::func::ConstStaticPrimFunc;
use crate::semantics::func::FreeCellFnExt;
use crate::semantics::func::FreeCellPrimFunc;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FreeStaticImpl;
use crate::semantics::func::FreeStaticPrimFunc;
use crate::semantics::func::FuncMode;
use crate::semantics::func::MutCellFnExt;
use crate::semantics::func::MutCellPrimFunc;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::MutStaticImpl;
use crate::semantics::func::MutStaticPrimFunc;
use crate::semantics::mode::Mode;
use crate::semantics::mode::SymbolMode;
use crate::semantics::val::ConstCellPrimFuncVal;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeCellPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::MutCellPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Map;
use crate::type_::Symbol;

thread_local!(pub(crate) static PRELUDE: RefCell<Box<dyn Prelude>> = RefCell::new(Box::new(EmptyPrelude)));

pub trait Prelude {
    fn put(&self, ctx: &mut dyn PreludeCtx);
}

pub trait PreludeCtx {
    fn put(&mut self, name: Symbol, val: Val);
}

struct EmptyPrelude;

impl Prelude for EmptyPrelude {
    fn put(&self, _ctx: &mut dyn PreludeCtx) {}
}

pub(crate) fn set_prelude(prelude: Box<dyn Prelude>) {
    PRELUDE.with(|p| {
        let Ok(mut p) = p.try_borrow_mut() else {
            return;
        };
        *p = prelude;
    });
}

pub(crate) fn initial_ctx() -> Ctx {
    let mut variables: Map<Symbol, CtxValue> = Map::default();
    put_preludes(&mut variables);
    let variables = CtxMap::new(variables);
    Ctx::new(variables)
}

pub(crate) fn put_preludes(ctx: &mut dyn PreludeCtx) {
    PRELUDE.with_borrow(|prelude| {
        prelude.put(ctx);
    });
}

pub(crate) fn find_prelude(id: Symbol) -> Option<Val> {
    let mut find = Find { name: id, val: None };
    PRELUDE.with_borrow(|prelude| {
        prelude.put(&mut find);
    });
    if find.val.is_some() {
        return find.val;
    }
    find.val
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

struct Find {
    name: Symbol,
    val: Option<Val>,
}

impl PreludeCtx for Find {
    fn put(&mut self, name: Symbol, val: Val) {
        if name == self.name {
            self.val = Some(val);
        }
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

impl<F: FreeCellFnExt + 'static> FreeFn<F> {
    pub fn free_cell(self) -> FreeCellPrimFuncVal {
        let func = FreeCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            mode: self.mode,
        };
        FreeCellPrimFuncVal::from(func)
    }
}

impl<F: FreeStaticFn<Val, Val> + 'static> FreeFn<F> {
    pub fn free_static(self) -> FreeStaticPrimFuncVal {
        let func = FreeStaticPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            mode: self.mode,
        };
        FreeStaticPrimFuncVal::from(func)
    }
}

impl<F: ConstCellFnExt + 'static> DynFn<F> {
    pub fn const_cell(self) -> ConstCellPrimFuncVal {
        let func = ConstCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            mode: self.mode,
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
            mode: self.mode,
            ctx_explicit: self.ctx_explicit,
        };
        ConstStaticPrimFuncVal::from(func)
    }
}

impl<F: MutCellFnExt + 'static> DynFn<F> {
    pub fn mut_cell(self) -> MutCellPrimFuncVal {
        let func = MutCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            mode: self.mode,
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
            mode: self.mode,
            ctx_explicit: self.ctx_explicit,
        };
        MutStaticPrimFuncVal::from(func)
    }
}

fn ctx_put<V: Clone + Into<Val>>(ctx: &mut dyn PreludeCtx, name: &'static str, val: &V) {
    let name = Symbol::from_str_unchecked(name);
    let val = val.clone().into();
    ctx.put(name, val);
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

pub(crate) fn ctx_default_mode() -> Option<Mode> {
    FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

pub(crate) fn ref_mode() -> Option<Mode> {
    let ref_ = MODE_PRELUDE.with(|p| p.ref_mode.clone());
    Some(Mode::Func(ref_.into()))
}

mod mode;

// -----

mod unit;

mod bit;

mod symbol;

mod text;

mod int;

mod number;

mod byte;

mod pair;

mod call;

mod list;

mod map;

mod ctx;

mod func;

// -----

mod solve;

mod meta;

mod syntax;

mod value;

mod ctrl;
