use airlang::{
    Call,
    CallMode,
    CtxConstFn,
    CtxFreeFn,
    CtxMutableFn,
    Func,
    FuncVal,
    InvariantTag,
    List,
    ListItemMode,
    ListMode,
    Map,
    MapMode,
    Mode,
    MutableCtx,
    Pair,
    Reverse,
    ReverseMode,
    Symbol,
    Transform,
    TransformMode,
    Val,
    ValMode,
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
    input_mode: TransformMode,
    output_mode: TransformMode,
    func: impl CtxFreeFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_free(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_const_fn(
    name: &'static str,
    input_mode: TransformMode,
    output_mode: TransformMode,
    func: impl CtxConstFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_const(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_mutable_fn(
    name: &'static str,
    input_mode: TransformMode,
    output_mode: TransformMode,
    func: impl CtxMutableFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_mutable(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn default_mode() -> TransformMode {
    TransformMode::default()
}

fn symbol_id_mode() -> TransformMode {
    let mode = ValMode {
        symbol: Transform::Id,
        ..Default::default()
    };
    Mode::Specific(mode)
}

#[allow(unused)]
fn pair_mode(first: TransformMode, second: TransformMode) -> TransformMode {
    let mode = ValMode {
        pair: Box::new(Pair::new(first, second)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

#[allow(unused)]
fn call_mode(func: TransformMode, input: TransformMode) -> TransformMode {
    let mode = ValMode {
        call: Mode::new(CallMode::ForAll(Call::new(func, input))),
        ..Default::default()
    };
    Mode::Specific(mode)
}

#[allow(unused)]
fn reverse_mode(func: TransformMode, output: TransformMode) -> TransformMode {
    let mode = ValMode {
        reverse: Mode::new(ReverseMode::ForAll(Reverse::new(func, output))),
        ..Default::default()
    };
    Mode::Specific(mode)
}

#[allow(unused)]
fn list_for_all_mode(mode: TransformMode) -> TransformMode {
    let mode = ValMode {
        list: Box::new(ListMode::ForAll(mode)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

#[allow(unused)]
fn list_for_some_mode(list_item: List<ListItemMode>) -> TransformMode {
    let mode = ValMode {
        list: Box::new(ListMode::ForSome(list_item)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

#[allow(unused)]
fn map_for_all_mode(key: TransformMode, value: TransformMode) -> TransformMode {
    let mode = ValMode {
        map: Box::new(MapMode::ForAll(Pair::new(key, value))),
        ..Default::default()
    };
    Mode::Specific(mode)
}

fn map_for_some_mode(map_mode: Map<Val, TransformMode>) -> TransformMode {
    let mode = ValMode {
        map: Box::new(MapMode::ForSome(map_mode)),
        ..Default::default()
    };
    Mode::Specific(mode)
}

pub(crate) mod io;

pub(crate) mod file;

pub(crate) mod process;

pub(crate) mod build;
