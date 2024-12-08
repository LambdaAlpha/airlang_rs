use std::rc::Rc;

use airlang::{
    CellFnExt,
    CellFunc,
    CellFuncVal,
    CompositeMode,
    ConstFn,
    ConstFunc,
    ConstFuncVal,
    FreeFn,
    FreeFunc,
    FreeFuncVal,
    FuncVal,
    Invariant,
    List,
    ListMode,
    Map,
    MapMode,
    Mode,
    MutCtx,
    MutFn,
    MutFunc,
    MutFuncVal,
    Pair,
    PairMode,
    PrimitiveMode,
    Symbol,
    Val,
};

use crate::prelude::{
    build::BuildPrelude,
    file::FilePrelude,
    io::IoPrelude,
    process::ProcessPrelude,
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default)]
pub(crate) struct AllPrelude {
    pub(crate) io: IoPrelude,
    pub(crate) file: FilePrelude,
    pub(crate) process: ProcessPrelude,
    pub(crate) build: BuildPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, mut ctx: MutCtx) {
        self.io.put(ctx.reborrow());
        self.file.put(ctx.reborrow());
        self.process.put(ctx.reborrow());
        self.build.put(ctx.reborrow());
    }
}

pub(crate) trait Prelude {
    fn put(&self, ctx: MutCtx);
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
fn named_cell_fn(
    name: &'static str,
    call_mode: Mode,
    abstract_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl CellFnExt + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = CellFunc::new(
        call_mode,
        abstract_mode,
        ask_mode,
        cacheable,
        name_symbol,
        Box::new(func),
    );
    let func_val = CellFuncVal::from(func);
    Named::new(name, FuncVal::Cell(func_val))
}

fn named_free_fn(
    name: &'static str,
    call_mode: Mode,
    abstract_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl FreeFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = FreeFunc::new(
        call_mode,
        abstract_mode,
        ask_mode,
        cacheable,
        name_symbol,
        Rc::new(func),
    );
    let func_val = FreeFuncVal::from(func);
    Named::new(name, FuncVal::Free(func_val))
}

fn named_const_fn(
    name: &'static str,
    call_mode: Mode,
    abstract_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl ConstFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = ConstFunc::new(
        call_mode,
        abstract_mode,
        ask_mode,
        cacheable,
        name_symbol,
        Rc::new(func),
    );
    let func_val = ConstFuncVal::from(func);
    Named::new(name, FuncVal::Const(func_val))
}

fn named_mut_fn(
    name: &'static str,
    call_mode: Mode,
    abstract_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl MutFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = MutFunc::new(
        call_mode,
        abstract_mode,
        ask_mode,
        cacheable,
        name_symbol,
        Rc::new(func),
    );
    let func_val = MutFuncVal::from(func);
    Named::new(name, FuncVal::Mut(func_val))
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

pub(crate) mod io;

pub(crate) mod file;

pub(crate) mod process;

pub(crate) mod build;
