use std::rc::Rc;

use airlang::{
    CompositeMode,
    ConstStaticFn,
    ConstStaticPrimFunc,
    ConstStaticPrimFuncVal,
    FreeCellFnExt,
    FreeCellPrimFunc,
    FreeCellPrimFuncVal,
    FreeStaticFn,
    FreeStaticPrimFunc,
    FreeStaticPrimFuncVal,
    FuncMode,
    FuncVal,
    Invariant,
    List,
    ListMode,
    Map,
    MapMode,
    Mode,
    MutCtx,
    MutStaticFn,
    MutStaticPrimFunc,
    MutStaticPrimFuncVal,
    Pair,
    PairMode,
    PrimitiveMode,
    Symbol,
    Val,
};

use crate::prelude::{
    eval::EvalPrelude,
    process::ProcessPrelude,
    repl::ReplPrelude,
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default)]
pub(crate) struct AllPrelude {
    pub(crate) repl: ReplPrelude,
    pub(crate) eval: EvalPrelude,
    pub(crate) process: ProcessPrelude,
}

pub(crate) trait Prelude {
    fn put(&self, ctx: MutCtx);
}

impl Prelude for AllPrelude {
    fn put(&self, mut ctx: MutCtx) {
        self.repl.put(ctx.reborrow());
        self.eval.put(ctx.reborrow());
        self.process.put(ctx.reborrow());
    }
}

#[derive(Clone)]
pub(crate) struct Named<T> {
    pub(crate) name: &'static str,
    pub(crate) value: T,
}

impl<T> Named<T> {
    pub(crate) fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
}

impl<T: Into<Val> + Clone> Named<T> {
    pub(crate) fn put(&self, ctx: MutCtx) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        let val = self.value.clone().into();
        ctx.put(name, Invariant::None, val)
            .expect("the name of preludes should be unique");
    }
}

#[allow(unused)]
fn named_free_cell_fn(
    name: &'static str,
    func: impl FreeCellFnExt + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = FreeCellPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = FreeCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::FreeCellPrim(func_val))
}

fn named_free_fn(
    name: &'static str,
    func: impl FreeStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = FreeStaticPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = FreeStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::FreeStaticPrim(func_val))
}

#[allow(unused)]
fn named_const_fn(
    name: &'static str,
    func: impl ConstStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
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
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = MutStaticPrimFunc::new(id, fn1, mode, cacheable);
    let func_val = MutStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::MutStaticPrim(func_val))
}

#[allow(unused)]
fn id_mode() -> Mode {
    Mode::Primitive(PrimitiveMode::Id)
}

fn form_mode() -> Mode {
    Mode::Primitive(PrimitiveMode::Form)
}

#[allow(unused)]
fn eval_mode() -> Mode {
    Mode::Primitive(PrimitiveMode::Eval)
}

#[allow(unused)]
fn pair_mode(first: Mode, second: Mode, default: PrimitiveMode) -> Mode {
    let mode = CompositeMode {
        pair: PairMode::Form(Pair::new(first, second)),
        ..CompositeMode::from(default)
    };
    Mode::Composite(Box::new(mode))
}

#[allow(unused)]
fn list_mode(head: List<Mode>, tail: Mode, default: PrimitiveMode) -> Mode {
    let mode = CompositeMode {
        list: ListMode::Form { head, tail },
        ..CompositeMode::from(default)
    };
    Mode::Composite(Box::new(mode))
}

fn map_mode(some: Map<Val, Mode>, key: Mode, value: Mode, default: PrimitiveMode) -> Mode {
    let else1 = Pair::new(key, value);
    let mode = CompositeMode {
        map: MapMode::Form { some, else1 },
        ..CompositeMode::from(default)
    };
    Mode::Composite(Box::new(mode))
}

mod repl;

mod eval;

mod process;
