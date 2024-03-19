use airlang::{
    Call,
    CallMode,
    CtxConstFn,
    CtxFreeFn,
    CtxMutableFn,
    EvalMode,
    Func,
    FuncVal,
    InvariantTag,
    IoMode,
    List,
    ListItemMode,
    ListMode,
    Map,
    MapMode,
    MatchMode,
    MutableCtx,
    Pair,
    PairMode,
    Reverse,
    ReverseMode,
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
    fn put(&self, mut ctx: MutableCtx) {
        self.io.put(ctx.reborrow());
        self.file.put(ctx.reborrow());
        self.process.put(ctx.reborrow());
        self.build.put(ctx.reborrow());
    }
}

pub(crate) trait Prelude {
    fn put(&self, ctx: MutableCtx);
}

#[derive(Clone)]
pub(crate) struct Named<T> {
    pub(crate) name: &'static str,
    pub(crate) value: T,
}

#[allow(unused)]
impl<T> Named<T> {
    pub(crate) fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
}

impl<T: Into<Val> + Clone> Named<T> {
    pub(crate) fn put(&self, mut ctx: MutableCtx) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        let val = self.value.clone().into();
        let _ = ctx.put(name, InvariantTag::Const, val);
    }
}

fn named_free_fn(
    name: &'static str,
    input_mode: IoMode,
    output_mode: IoMode,
    func: impl CtxFreeFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_free(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_const_fn(
    name: &'static str,
    input_mode: IoMode,
    output_mode: IoMode,
    func: impl CtxConstFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_const(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_mutable_fn(
    name: &'static str,
    input_mode: IoMode,
    output_mode: IoMode,
    func: impl CtxMutableFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_mutable(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn default_mode() -> IoMode {
    IoMode::default()
}

fn symbol_id_mode() -> IoMode {
    let mode = MatchMode {
        symbol: EvalMode::Id,
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn pair_mode(first: IoMode, second: IoMode) -> IoMode {
    let mode = MatchMode {
        pair: Box::new(PairMode::Pair(Pair::new(first, second))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn call_mode(func: IoMode, input: IoMode) -> IoMode {
    let mode = MatchMode {
        call: Box::new(CallMode::Call(Call::new(func, input))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn reverse_mode(func: IoMode, output: IoMode) -> IoMode {
    let mode = MatchMode {
        reverse: Box::new(ReverseMode::Reverse(Reverse::new(func, output))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn list_mode(list_mode: ListMode) -> IoMode {
    let mode = MatchMode {
        list: Box::new(list_mode),
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn list_mode_for_all(mode: IoMode) -> IoMode {
    let mode = MatchMode {
        list: Box::new(ListMode::ForAll(mode)),
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn list_mode_for_some(list_item: List<ListItemMode>) -> IoMode {
    let mode = MatchMode {
        list: Box::new(ListMode::ForSome(list_item)),
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn map_mode(map_mode: MapMode) -> IoMode {
    let mode = MatchMode {
        map: Box::new(map_mode),
        ..Default::default()
    };
    IoMode::Match(mode)
}

#[allow(unused)]
fn map_mode_for_all(key: IoMode, value: IoMode) -> IoMode {
    let mode = MatchMode {
        map: Box::new(MapMode::ForAll(Pair::new(key, value))),
        ..Default::default()
    };
    IoMode::Match(mode)
}

fn map_mode_for_some(map_mode: Map<Val, IoMode>) -> IoMode {
    let mode = MatchMode {
        map: Box::new(MapMode::ForSome(map_mode)),
        ..Default::default()
    };
    IoMode::Match(mode)
}

pub(crate) mod io;

pub(crate) mod file;

pub(crate) mod process;

pub(crate) mod build;
