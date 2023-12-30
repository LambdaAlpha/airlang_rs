use std::rc::Rc;

use airlang::{
    Call,
    CallMode,
    EvalMode,
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
    ValExt,
};
use airlang_ext::{
    ExtFunc,
    ExtFuncVal,
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

#[allow(unused)]
impl<T> Named<T> {
    pub(crate) fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
}

#[allow(unused)]
impl<T: ValExt + 'static> Named<T> {
    pub(crate) fn put(self, mut ctx: MutableCtx) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        let val = Val::Ext(Box::new(self.value));
        let _ = ctx.put(name, InvariantTag::Const, val);
    }
}

pub(crate) fn put_func(func: &Rc<ExtFunc>, mut ctx: MutableCtx) {
    let id = func.id().clone();
    let val = Val::Ext(Box::new(ExtFuncVal::from(func.clone())));
    let _ = ctx.put(id, InvariantTag::Const, val);
}

fn default_mode() -> IoMode {
    IoMode::default()
}

#[allow(unused)]
fn symbol_value_mode() -> IoMode {
    let mode = MatchMode {
        symbol: EvalMode::Value,
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

mod repl;

mod eval;

mod process;
