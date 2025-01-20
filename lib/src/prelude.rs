use std::rc::Rc;

use crate::{
    CompMode,
    ConstStaticPrimFuncVal,
    FreeCellPrimFunc,
    FreeCellPrimFuncVal,
    FreeStaticPrimFunc,
    FreeStaticPrimFuncVal,
    Id,
    List,
    Map,
    MutStaticPrimFunc,
    MutStaticPrimFuncVal,
    Pair,
    PairMode,
    SymbolMode,
    ctx::{
        Ctx,
        CtxValue,
        map::CtxMap,
    },
    func::{
        FuncMode,
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
        mut_cell_prim::{
            MutCellFnExt,
            MutCellPrimFunc,
        },
        mut_static_prim::MutStaticFn,
    },
    mode::{
        Mode,
        eval::Eval,
        form::Form,
        list::ListMode,
        map::MapMode,
        symbol::PrefixMode,
        united::UniMode,
    },
    prelude::{
        abstract1::AbstractPrelude,
        ask::AskPrelude,
        bit::BitPrelude,
        byte::BytePrelude,
        call::CallPrelude,
        case::CasePrelude,
        ctrl::CtrlPrelude,
        ctx::CtxPrelude,
        extension::ExtPrelude,
        func::FuncPrelude,
        int::IntPrelude,
        list::ListPrelude,
        map::MapPrelude,
        meta::MetaPrelude,
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
    pub(crate) list: ListPrelude,
    pub(crate) map: MapPrelude,
    pub(crate) ctx: CtxPrelude,
    pub(crate) func: FuncPrelude,
    pub(crate) case: CasePrelude,
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
        self.list.put(m);
        self.map.put(m);
        self.ctx.put(m);
        self.func.put(m);
        self.case.put(m);
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

#[allow(unused)]
fn named_free_cell_fn(
    name: &'static str,
    func: impl FreeCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = FreeCellPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = FreeCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::FreeCellPrim(func_val))
}

#[allow(unused)]
fn named_const_cell_fn(
    name: &'static str,
    func: impl ConstCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = ConstCellPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = ConstCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::ConstCellPrim(func_val))
}

#[allow(unused)]
fn named_mut_cell_fn(
    name: &'static str,
    func: impl MutCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = Symbol::from_str(name);
    let fn1 = Box::new(func);
    let func = MutCellPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = MutCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::MutCellPrim(func_val))
}

fn named_free_fn(
    name: &'static str,
    func: impl FreeStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = FreeStaticPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = FreeStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::FreeStaticPrim(func_val))
}

fn named_const_fn(
    name: &'static str,
    func: impl ConstStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = ConstStaticPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = ConstStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::ConstStaticPrim(func_val))
}

fn named_mut_fn(
    name: &'static str,
    func: impl MutStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = Symbol::from_str(name);
    let fn1 = Rc::new(func);
    let func = MutStaticPrimFunc::new_inner(id, fn1, mode, cacheable);
    let func_val = MutStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::MutStaticPrim(func_val))
}

pub(crate) fn id_mode() -> Mode {
    Mode::Uni(UniMode::Id(Id))
}

pub(crate) fn form_mode(prefix_mode: PrefixMode) -> Mode {
    Mode::Uni(UniMode::Form(Form::new(prefix_mode)))
}

#[allow(unused)]
pub(crate) fn eval_mode(prefix_mode: PrefixMode) -> Mode {
    Mode::Uni(UniMode::Eval(Eval::new(prefix_mode)))
}

pub(crate) fn symbol_literal_mode() -> Mode {
    let mode = CompMode {
        symbol: SymbolMode::Form(PrefixMode::Literal),
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

pub(crate) fn pair_mode(first: Mode, second: Mode) -> Mode {
    let mode = CompMode {
        pair: PairMode::Form(Pair::new(first, second)),
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

#[allow(unused)]
pub(crate) fn list_mode(head: List<Mode>, tail: Mode) -> Mode {
    let mode = CompMode {
        list: ListMode::Form { head, tail },
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

pub(crate) fn map_mode(some: Map<Val, Mode>, key: Mode, value: Mode) -> Mode {
    let else1 = Pair::new(key, value);
    let mode = CompMode {
        map: MapMode::Form { some, else1 },
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

pub(crate) fn ref_pair_mode() -> Mode {
    pair_mode(id_mode(), Mode::default())
}

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

mod list;

mod map;

mod ctx;

mod func;

mod case;

mod extension;

mod meta;

mod syntax;

mod value;

mod ctrl;
