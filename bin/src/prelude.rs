use airlang::{
    Call,
    CallMode,
    CtxConstFn,
    CtxFreeFn,
    CtxMutableFn,
    Func,
    FuncVal,
    Invariant,
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
    SymbolMode,
    Val,
    ValMode,
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
    fn put(&self, ctx: MutableCtx);
}

impl Prelude for AllPrelude {
    fn put(&self, mut ctx: MutableCtx) {
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
    pub(crate) fn put(&self, mut ctx: MutableCtx) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        let val = self.value.clone().into();
        let _ = ctx.put(name, Invariant::Const, val);
    }
}

fn named_free_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxFreeFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_free(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

#[allow(unused)]
fn named_const_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxConstFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_const(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_mutable_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxMutableFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_mutable(input_mode, output_mode, name_symbol, Box::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn default_mode() -> Mode {
    Mode::default()
}

#[allow(unused)]
fn symbol_id_mode() -> Mode {
    let mode = ValMode {
        symbol: SymbolMode::Id,
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

#[allow(unused)]
fn pair_mode(first: Mode, second: Mode) -> Mode {
    let mode = ValMode {
        pair: Box::new(Pair::new(first, second)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

#[allow(unused)]
fn call_mode(func: Mode, input: Mode) -> Mode {
    let mode = ValMode {
        call: Box::new(CallMode::Struct(Call::new(func, input))),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

#[allow(unused)]
fn reverse_mode(func: Mode, output: Mode) -> Mode {
    let mode = ValMode {
        reverse: Box::new(ReverseMode::Struct(Reverse::new(func, output))),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

#[allow(unused)]
fn list_all_mode(mode: Mode) -> Mode {
    let mode = ValMode {
        list: Box::new(ListMode::All(mode)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

#[allow(unused)]
fn list_some_mode(list_item: List<ListItemMode>) -> Mode {
    let mode = ValMode {
        list: Box::new(ListMode::Some(list_item)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

#[allow(unused)]
fn map_all_mode(key: Mode, value: Mode) -> Mode {
    let mode = ValMode {
        map: Box::new(MapMode::All(Pair::new(key, value))),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

fn map_some_mode(map_mode: Map<Val, Mode>) -> Mode {
    let mode = ValMode {
        map: Box::new(MapMode::Some(map_mode)),
        ..Default::default()
    };
    Mode::Custom(Box::new(mode))
}

mod repl;

mod eval;

mod process;
