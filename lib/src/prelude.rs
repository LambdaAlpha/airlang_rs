use std::rc::Rc;

use crate::ConstRef;
use crate::ConstStaticImpl;
use crate::ConstStaticPrimFuncVal;
use crate::FreeCellPrimFunc;
use crate::FreeCellPrimFuncVal;
use crate::FreeStaticImpl;
use crate::FreeStaticPrimFunc;
use crate::FreeStaticPrimFuncVal;
use crate::FuncVal;
use crate::Map;
use crate::Mode;
use crate::MutStaticImpl;
use crate::MutStaticPrimFunc;
use crate::MutStaticPrimFuncVal;
use crate::SymbolMode;
use crate::ctx::Ctx;
use crate::ctx::map::Contract;
use crate::ctx::map::CtxMap;
use crate::ctx::map::CtxValue;
use crate::extension::AIR_EXT;
use crate::func::const_cell_prim::ConstCellFnExt;
use crate::func::const_cell_prim::ConstCellPrimFunc;
use crate::func::const_static_prim::ConstStaticFn;
use crate::func::const_static_prim::ConstStaticPrimFunc;
use crate::func::free_cell_prim::FreeCellFnExt;
use crate::func::free_static_prim::FreeStaticFn;
use crate::func::func_mode::FuncMode;
use crate::func::mut_cell_prim::MutCellFnExt;
use crate::func::mut_cell_prim::MutCellPrimFunc;
use crate::func::mut_static_prim::MutStaticFn;
use crate::prelude::bit::BitPrelude;
use crate::prelude::byte::BytePrelude;
use crate::prelude::call::CallPrelude;
use crate::prelude::ctrl::CtrlPrelude;
use crate::prelude::ctx::CtxPrelude;
use crate::prelude::extension::ExtPrelude;
use crate::prelude::func::FuncPrelude;
use crate::prelude::int::IntPrelude;
use crate::prelude::list::ListPrelude;
use crate::prelude::map::MapPrelude;
use crate::prelude::meta::MetaPrelude;
use crate::prelude::mode::MODE_PRELUDE;
use crate::prelude::number::NumberPrelude;
use crate::prelude::pair::PairPrelude;
use crate::prelude::solve::SolvePrelude;
use crate::prelude::symbol::SymbolPrelude;
use crate::prelude::syntax::SyntaxPrelude;
use crate::prelude::text::TextPrelude;
use crate::prelude::unit::UnitPrelude;
use crate::prelude::value::ValuePrelude;
use crate::symbol::Symbol;
use crate::val::Val;
use crate::val::func::const_cell_prim::ConstCellPrimFuncVal;
use crate::val::func::mut_cell_prim::MutCellPrimFuncVal;

pub trait Prelude {
    fn put(&self, ctx: &mut dyn PreludeCtx);
}

pub trait PreludeCtx {
    fn put(&mut self, name: Symbol, val: Val);
}

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default, Clone)]
pub(crate) struct AllPrelude {
    pub(crate) unit: UnitPrelude,
    pub(crate) bool: BitPrelude,
    pub(crate) symbol: SymbolPrelude,
    pub(crate) text: TextPrelude,
    pub(crate) int: IntPrelude,
    pub(crate) number: NumberPrelude,
    pub(crate) byte: BytePrelude,
    pub(crate) pair: PairPrelude,
    pub(crate) call: CallPrelude,
    pub(crate) list: ListPrelude,
    pub(crate) map: MapPrelude,
    pub(crate) ctx: CtxPrelude,
    pub(crate) func: FuncPrelude,
    pub(crate) extension: ExtPrelude,
    pub(crate) solve: SolvePrelude,
    pub(crate) meta: MetaPrelude,
    pub(crate) syntax: SyntaxPrelude,
    pub(crate) value: ValuePrelude,
    pub(crate) ctrl: CtrlPrelude,
}

impl Prelude for AllPrelude {
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
        self.extension.put(ctx);
        self.solve.put(ctx);
        self.meta.put(ctx);
        self.syntax.put(ctx);
        self.value.put(ctx);
        self.ctrl.put(ctx);
    }
}

pub(crate) fn initial_ctx() -> Ctx {
    let mut variables: Map<Symbol, CtxValue> = Map::default();
    put_preludes(&mut variables);
    let variables = CtxMap::new(variables);
    Ctx::new(variables)
}

pub(crate) fn put_preludes(ctx: &mut dyn PreludeCtx) {
    PRELUDE.with(|prelude| {
        prelude.put(ctx);
    });
    AIR_EXT.with_borrow(|prelude| {
        prelude.put(ctx);
    });
}

pub(crate) fn find_prelude(id: Symbol) -> Option<Val> {
    let mut find = Find { name: id, val: None };
    PRELUDE.with(|prelude| {
        prelude.put(&mut find);
    });
    if find.val.is_some() {
        return find.val;
    }
    AIR_EXT.with_borrow(|prelude| {
        prelude.put(&mut find);
    });
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

pub(crate) struct FreeFn<F> {
    pub(crate) id: &'static str,
    pub(crate) f: F,
    pub(crate) mode: FuncMode,
}

pub(crate) struct DynFn<F> {
    pub(crate) id: &'static str,
    pub(crate) f: F,
    pub(crate) mode: FuncMode,
    pub(crate) ctx_explicit: bool,
}

impl<F: FreeCellFnExt + 'static> FreeFn<F> {
    #[expect(dead_code)]
    pub(crate) fn free_cell(self) -> FreeCellPrimFuncVal {
        let func = FreeCellPrimFunc {
            id: Symbol::from_str(self.id),
            extension: false,
            fn1: Box::new(self.f),
            mode: self.mode,
        };
        FreeCellPrimFuncVal::from(func)
    }
}

impl<F: FreeStaticFn<Val, Val> + 'static> FreeFn<F> {
    pub(crate) fn free_static(self) -> FreeStaticPrimFuncVal {
        let func = FreeStaticPrimFunc {
            id: Symbol::from_str(self.id),
            extension: false,
            fn1: Rc::new(self.f),
            mode: self.mode,
        };
        FreeStaticPrimFuncVal::from(func)
    }
}

impl<F: ConstCellFnExt + 'static> DynFn<F> {
    #[expect(dead_code)]
    pub(crate) fn const_cell(self) -> ConstCellPrimFuncVal {
        let func = ConstCellPrimFunc {
            id: Symbol::from_str(self.id),
            extension: false,
            fn1: Box::new(self.f),
            mode: self.mode,
            ctx_explicit: self.ctx_explicit,
        };
        ConstCellPrimFuncVal::from(func)
    }
}

impl<F: ConstStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    pub(crate) fn const_static(self) -> ConstStaticPrimFuncVal {
        let func = ConstStaticPrimFunc {
            id: Symbol::from_str(self.id),
            extension: false,
            fn1: Rc::new(self.f),
            mode: self.mode,
            ctx_explicit: self.ctx_explicit,
        };
        ConstStaticPrimFuncVal::from(func)
    }
}

impl<F: MutCellFnExt + 'static> DynFn<F> {
    #[expect(dead_code)]
    pub(crate) fn mut_cell(self) -> MutCellPrimFuncVal {
        let func = MutCellPrimFunc {
            id: Symbol::from_str(self.id),
            extension: false,
            fn1: Box::new(self.f),
            mode: self.mode,
            ctx_explicit: self.ctx_explicit,
        };
        MutCellPrimFuncVal::from(func)
    }
}

impl<F: MutStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    pub(crate) fn mut_static(self) -> MutStaticPrimFuncVal {
        let func = MutStaticPrimFunc {
            id: Symbol::from_str(self.id),
            extension: false,
            fn1: Rc::new(self.f),
            mode: self.mode,
            ctx_explicit: self.ctx_explicit,
        };
        MutStaticPrimFuncVal::from(func)
    }
}

fn ctx_put<V: Clone + Into<Val>>(ctx: &mut dyn PreludeCtx, name: &'static str, val: &V) {
    let name = Symbol::from_str(name);
    let val = val.clone().into();
    ctx.put(name, val);
}

fn free_impl(func: fn(Val) -> Val) -> FreeStaticImpl<Val, Val> {
    FreeStaticImpl::new(func)
}

fn const_impl(func: fn(ConstRef<Val>, Val) -> Val) -> ConstStaticImpl<Val, Val, Val> {
    ConstStaticImpl::new(FreeStaticImpl::default, func)
}

fn mut_impl(func: fn(&mut Val, Val) -> Val) -> MutStaticImpl<Val, Val, Val> {
    MutStaticImpl::new(FreeStaticImpl::default, ConstStaticImpl::default, func)
}

pub(crate) fn ctx_default_mode() -> Option<Mode> {
    FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

pub(crate) fn ref_mode() -> Option<Mode> {
    let ref1 = MODE_PRELUDE.with(|p| p.ref_mode.clone());
    Some(Mode::Func(ref1.into()))
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

mod extension;

// -----

mod solve;

mod meta;

mod syntax;

mod value;

mod ctrl;
