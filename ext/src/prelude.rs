use std::rc::Rc;

use airlang::{
    BasicMode,
    ConstFn,
    ConstFunc,
    ConstFuncVal,
    FreeFnExt,
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
    StaticFn,
    StaticFunc,
    StaticFuncVal,
    Symbol,
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
fn named_free_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl FreeFnExt + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = FreeFunc::new(call_mode, ask_mode, cacheable, name_symbol, Box::new(func));
    let func_val = FreeFuncVal::from(func);
    Named::new(name, FuncVal::Free(func_val))
}

fn named_static_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl StaticFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = StaticFunc::new(call_mode, ask_mode, cacheable, name_symbol, Rc::new(func));
    let func_val = StaticFuncVal::from(func);
    Named::new(name, FuncVal::Static(func_val))
}

fn named_const_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl ConstFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = ConstFunc::new(call_mode, ask_mode, cacheable, name_symbol, Rc::new(func));
    let func_val = ConstFuncVal::from(func);
    Named::new(name, FuncVal::Const(func_val))
}

fn named_mut_fn(
    name: &'static str,
    call_mode: Mode,
    ask_mode: Mode,
    cacheable: bool,
    func: impl MutFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = MutFunc::new(call_mode, ask_mode, cacheable, name_symbol, Rc::new(func));
    let func_val = MutFuncVal::from(func);
    Named::new(name, FuncVal::Mut(func_val))
}

#[allow(unused)]
fn id_mode() -> Mode {
    Mode {
        default: BasicMode::Id,
        specialized: None,
    }
}

fn form_mode() -> Mode {
    Mode {
        default: BasicMode::Form,
        specialized: None,
    }
}

#[allow(unused)]
fn eval_mode() -> Mode {
    Mode {
        default: BasicMode::Eval,
        specialized: None,
    }
}

#[allow(unused)]
fn pair_mode(first: Mode, second: Mode, default: BasicMode) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        pair: Pair::new(first, second),
        list: ListMode {
            head: List::default(),
            tail: default_mode.clone(),
        },
        map: MapMode {
            some: Map::default(),
            else1: Pair::new(default_mode.clone(), default_mode),
        },
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

#[allow(unused)]
fn list_mode(head: List<Mode>, tail: Mode, default: BasicMode) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        list: ListMode { head, tail },
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        map: MapMode {
            some: Map::default(),
            else1: Pair::new(default_mode.clone(), default_mode),
        },
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

fn map_mode(some: Map<Val, Mode>, key: Mode, value: Mode, default: BasicMode) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let else1 = Pair::new(key, value);
    let val_mode = ValMode {
        map: MapMode { some, else1 },
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        list: ListMode {
            head: List::default(),
            tail: default_mode,
        },
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

pub(crate) mod io;

pub(crate) mod file;

pub(crate) mod process;

pub(crate) mod build;
