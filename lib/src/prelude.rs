use std::rc::Rc;

use crate::{
    ConstStaticPrimFuncVal,
    FreeCellPrimFunc,
    FreeCellPrimFuncVal,
    FreeStaticPrimFunc,
    FreeStaticPrimFuncVal,
    Map,
    Mode,
    MutStaticPrimFunc,
    MutStaticPrimFuncVal,
    ctx::{
        Ctx,
        map::{
            CtxMap,
            CtxValue,
        },
    },
    func::{
        const_cell_prim::{
            ConstCellFnExt,
            ConstCellPrimFunc,
        },
        const_static_prim::{
            ConstStaticFn,
            ConstStaticPrimFunc,
        },
        free_cell_prim::FreeCellFnExt,
        free_static_prim::FreeStaticFn,
        func_mode::FuncMode,
        mut_cell_prim::{
            MutCellFnExt,
            MutCellPrimFunc,
        },
        mut_static_prim::MutStaticFn,
    },
    prelude::{
        abstract1::AbstractPrelude,
        ask::AskPrelude,
        bit::BitPrelude,
        byte::BytePrelude,
        call::CallPrelude,
        change::ChangePrelude,
        ctrl::CtrlPrelude,
        ctx::CtxPrelude,
        extension::ExtPrelude,
        func::FuncPrelude,
        int::IntPrelude,
        list::ListPrelude,
        map::MapPrelude,
        meta::MetaPrelude,
        mode::MODE_PRELUDE,
        number::NumberPrelude,
        pair::PairPrelude,
        symbol::SymbolPrelude,
        syntax::SyntaxPrelude,
        text::TextPrelude,
        unit::UnitPrelude,
        value::ValuePrelude,
    },
    symbol::Symbol,
    val::{
        Val,
        func::{
            FuncVal,
            const_cell_prim::ConstCellPrimFuncVal,
            mut_cell_prim::MutCellPrimFuncVal,
        },
    },
};

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
    pub(crate) abstract1: AbstractPrelude,
    pub(crate) ask: AskPrelude,
    pub(crate) change: ChangePrelude,
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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.unit.put(m);
        self.bool.put(m);
        self.symbol.put(m);
        self.text.put(m);
        self.int.put(m);
        self.number.put(m);
        self.byte.put(m);
        self.pair.put(m);
        self.call.put(m);
        self.abstract1.put(m);
        self.ask.put(m);
        self.change.put(m);
        self.list.put(m);
        self.map.put(m);
        self.ctx.put(m);
        self.func.put(m);
        self.extension.put(m);
        self.meta.put(m);
        self.syntax.put(m);
        self.value.put(m);
        self.ctrl.put(m);
    }
}

pub(crate) fn initial_ctx() -> Ctx {
    let variables = PRELUDE.with(|prelude| {
        let mut m = Map::default();
        prelude.put(&mut m);
        m
    });
    let variables = CtxMap::new(variables, false);
    Ctx::new(variables, None)
}

pub(crate) trait Prelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>);
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
    pub(crate) fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        let name = Symbol::from_str(self.name);
        let value = CtxValue::new(self.value.clone().into());
        m.insert(name, value);
    }
}

fn free_cell_fn(
    name: &'static str,
    func: impl FreeCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = FreeCellPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = FreeCellPrimFuncVal::from(func);
    FuncVal::FreeCellPrim(func_val)
}

#[allow(unused)]
fn named_free_cell_fn(
    name: &'static str,
    func: impl FreeCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let f = free_cell_fn(name, func, mode, cacheable);
    Named::new(name, f)
}

fn const_cell_fn(
    name: &'static str,
    func: impl ConstCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = ConstCellPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = ConstCellPrimFuncVal::from(func);
    FuncVal::ConstCellPrim(func_val)
}

#[allow(unused)]
fn named_const_cell_fn(
    name: &'static str,
    func: impl ConstCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let f = const_cell_fn(name, func, mode, cacheable);
    Named::new(name, f)
}

fn mut_cell_fn(
    name: &'static str,
    func: impl MutCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = MutCellPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = MutCellPrimFuncVal::from(func);
    FuncVal::MutCellPrim(func_val)
}

#[allow(unused)]
fn named_mut_cell_fn(
    name: &'static str,
    func: impl MutCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let f = mut_cell_fn(name, func, mode, cacheable);
    Named::new(name, f)
}

fn free_fn(
    name: &'static str,
    func: impl FreeStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = FreeStaticPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = FreeStaticPrimFuncVal::from(func);
    FuncVal::FreeStaticPrim(func_val)
}

fn named_free_fn(
    name: &'static str,
    func: impl FreeStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let f = free_fn(name, func, mode, cacheable);
    Named::new(name, f)
}

fn const_fn(
    name: &'static str,
    func: impl ConstStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = ConstStaticPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = ConstStaticPrimFuncVal::from(func);
    FuncVal::ConstStaticPrim(func_val)
}

fn named_const_fn(
    name: &'static str,
    func: impl ConstStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let f = const_fn(name, func, mode, cacheable);
    Named::new(name, f)
}

fn mut_fn(
    name: &'static str,
    func: impl MutStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> FuncVal {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = MutStaticPrimFunc::new_inner(id, fn1, mode, cacheable);
    let func_val = MutStaticPrimFuncVal::from(func);
    FuncVal::MutStaticPrim(func_val)
}

fn named_mut_fn(
    name: &'static str,
    func: impl MutStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let f = mut_fn(name, func, mode, cacheable);
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

mod abstract1;

mod ask;

mod change;

mod list;

mod map;

mod ctx;

mod func;

mod extension;

mod meta;

mod syntax;

mod value;

mod ctrl;
