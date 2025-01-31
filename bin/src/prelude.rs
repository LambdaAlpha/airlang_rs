use std::rc::Rc;

use airlang::{
    CompMode,
    ConstCellFnExt,
    ConstCellPrimFunc,
    ConstCellPrimFuncVal,
    ConstStaticFn,
    ConstStaticPrimFunc,
    ConstStaticPrimFuncVal,
    Eval,
    Form,
    FreeCellFnExt,
    FreeCellPrimFunc,
    FreeCellPrimFuncVal,
    FreeStaticFn,
    FreeStaticPrimFunc,
    FreeStaticPrimFuncVal,
    FuncMode,
    FuncVal,
    Id,
    List,
    ListMode,
    Map,
    MapMode,
    Mode,
    MutCellFnExt,
    MutCellPrimFunc,
    MutCellPrimFuncVal,
    MutCtx,
    MutStaticFn,
    MutStaticPrimFunc,
    MutStaticPrimFuncVal,
    Pair,
    PairMode,
    PrefixMode,
    Symbol,
    SymbolMode,
    UniMode,
    Val,
    VarAccess,
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
        ctx.put(name, VarAccess::Assign, val)
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
    let func = FreeCellPrimFunc::new_extension(id, fn1, mode, cacheable);
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
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = ConstCellPrimFunc::new_extension(id, fn1, mode, cacheable);
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
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = MutCellPrimFunc::new_extension(id, fn1, mode, cacheable);
    let func_val = MutCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::MutCellPrim(func_val))
}

fn named_free_fn(
    name: &'static str,
    func: impl FreeStaticFn + 'static,
    mode: FuncMode,
    cacheable: bool,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = FreeStaticPrimFunc::new_extension(id, fn1, mode, cacheable);
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
    let func = ConstStaticPrimFunc::new_extension(id, fn1, mode, cacheable);
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
    Mode::Uni(UniMode::Id(Id))
}

fn form_mode(prefix_mode: PrefixMode) -> Mode {
    Mode::Uni(UniMode::Form(Form::new(prefix_mode)))
}

#[allow(unused)]
fn eval_mode(prefix_mode: PrefixMode) -> Mode {
    Mode::Uni(UniMode::Eval(Eval::new(prefix_mode)))
}

#[allow(unused)]
fn symbol_literal_mode() -> Mode {
    let mode = CompMode {
        symbol: SymbolMode::Form(PrefixMode::Literal),
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

#[allow(unused)]
fn pair_mode(first: Mode, second: Mode) -> Mode {
    let mode = CompMode {
        pair: PairMode::Form(Pair::new(first, second)),
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

#[allow(unused)]
fn list_mode(head: List<Mode>, tail: Mode) -> Mode {
    let mode = CompMode {
        list: ListMode::Form { head, tail },
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

#[allow(unused)]
fn map_mode(some: Map<Val, Mode>, key: Mode, value: Mode) -> Mode {
    let else1 = Pair::new(key, value);
    let mode = CompMode {
        map: MapMode::Form { some, else1 },
        ..CompMode::from(UniMode::default())
    };
    Mode::Comp(Box::new(mode))
}

mod repl;

mod eval;

mod process;
