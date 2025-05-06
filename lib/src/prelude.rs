use std::rc::Rc;

use crate::ConstStaticPrimFuncVal;
use crate::FreeCellPrimFunc;
use crate::FreeCellPrimFuncVal;
use crate::FreeStaticPrimFunc;
use crate::FreeStaticPrimFuncVal;
use crate::Map;
use crate::Mode;
use crate::MutStaticPrimFunc;
use crate::MutStaticPrimFuncVal;
use crate::ctx::Ctx;
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
use crate::prelude::symbol::SymbolPrelude;
use crate::prelude::syntax::SyntaxPrelude;
use crate::prelude::text::TextPrelude;
use crate::prelude::unit::UnitPrelude;
use crate::prelude::value::ValuePrelude;
use crate::symbol::Symbol;
use crate::val::Val;
use crate::val::func::FuncVal;
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
        self.meta.put(ctx);
        self.syntax.put(ctx);
        self.value.put(ctx);
        self.ctrl.put(ctx);
    }
}

pub(crate) fn initial_ctx() -> Ctx {
    let mut variables: Map<Symbol, CtxValue> = Map::default();
    put_preludes(&mut variables);
    let variables = CtxMap::new(variables, false);
    Ctx::new(variables, None)
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
        let v = self.insert(name, CtxValue::new(val));
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

#[derive(Clone)]
pub(crate) struct Named<T> {
    pub(crate) name: &'static str,
    pub(crate) value: T,
}

impl<T> Named<T> {
    pub(crate) fn new(name: &'static str, value: T) -> Named<T> {
        Named { name, value }
    }
}

impl<T: Into<Val> + Clone> Named<T> {
    pub(crate) fn put(&self, m: &mut dyn PreludeCtx) {
        let name = Symbol::from_str(self.name);
        let value = self.value.clone().into();
        m.put(name, value);
    }
}

fn free_cell_fn(name: &'static str, func: impl FreeCellFnExt + 'static, mode: FuncMode) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = FreeCellPrimFunc::new(id, fn1, mode);
    let func_val = FreeCellPrimFuncVal::from(func);
    FuncVal::FreeCellPrim(func_val)
}

#[expect(dead_code)]
fn named_free_cell_fn(
    name: &'static str, func: impl FreeCellFnExt + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = free_cell_fn(name, func, mode);
    Named::new(name, f)
}

fn const_cell_fn(
    name: &'static str, func: impl ConstCellFnExt + 'static, mode: FuncMode,
) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = ConstCellPrimFunc::new(id, fn1, mode);
    let func_val = ConstCellPrimFuncVal::from(func);
    FuncVal::ConstCellPrim(func_val)
}

#[expect(dead_code)]
fn named_const_cell_fn(
    name: &'static str, func: impl ConstCellFnExt + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = const_cell_fn(name, func, mode);
    Named::new(name, f)
}

fn mut_cell_fn(name: &'static str, func: impl MutCellFnExt + 'static, mode: FuncMode) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = MutCellPrimFunc::new(id, fn1, mode);
    let func_val = MutCellPrimFuncVal::from(func);
    FuncVal::MutCellPrim(func_val)
}

#[expect(dead_code)]
fn named_mut_cell_fn(
    name: &'static str, func: impl MutCellFnExt + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = mut_cell_fn(name, func, mode);
    Named::new(name, f)
}

fn free_fn(name: &'static str, func: impl FreeStaticFn + 'static, mode: FuncMode) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = FreeStaticPrimFunc::new(id, fn1, mode);
    let func_val = FreeStaticPrimFuncVal::from(func);
    FuncVal::FreeStaticPrim(func_val)
}

fn named_free_fn(
    name: &'static str, func: impl FreeStaticFn + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = free_fn(name, func, mode);
    Named::new(name, f)
}

fn const_fn(name: &'static str, func: impl ConstStaticFn + 'static, mode: FuncMode) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = ConstStaticPrimFunc::new(id, fn1, mode);
    let func_val = ConstStaticPrimFuncVal::from(func);
    FuncVal::ConstStaticPrim(func_val)
}

fn named_const_fn(
    name: &'static str, func: impl ConstStaticFn + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = const_fn(name, func, mode);
    Named::new(name, f)
}

fn mut_fn(name: &'static str, func: impl MutStaticFn + 'static, mode: FuncMode) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = MutStaticPrimFunc::new(id, fn1, mode);
    let func_val = MutStaticPrimFuncVal::from(func);
    FuncVal::MutStaticPrim(func_val)
}

fn named_mut_fn(
    name: &'static str, func: impl MutStaticFn + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = mut_fn(name, func, mode);
    Named::new(name, f)
}

pub(crate) fn ref_pair_mode() -> Option<Mode> {
    FuncMode::pair_mode(ref_mode(), FuncMode::default_mode())
}

pub(crate) fn ref_mode() -> Option<Mode> {
    let ref1 = MODE_PRELUDE.with(|p| p.ref_mode.clone());
    Some(Mode::Func(ref1))
}

mod mode;

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

mod meta;

mod syntax;

mod value;

mod ctrl;
