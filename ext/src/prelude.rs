use std::rc::Rc;

use airlang::{
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
    Symbol,
    Transform,
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

impl<T> Named<T> {
    pub(crate) fn new(name: &'static str, value: T) -> Self {
        Self { name, value }
    }
}

impl<T: Into<Val> + Clone> Named<T> {
    pub(crate) fn put(&self, ctx: MutableCtx) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        let val = self.value.clone().into();
        ctx.put(name, Invariant::Const, val)
            .expect("the name of preludes should be unique");
    }
}

fn named_free_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxFreeFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_free(input_mode, output_mode, name_symbol, Rc::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

fn named_const_fn(
    name: &'static str,
    input_mode: Mode,
    output_mode: Mode,
    func: impl CtxConstFn + 'static,
) -> Named<FuncVal> {
    let name_symbol = unsafe { Symbol::from_str_unchecked(name) };
    let func = Func::new_const(input_mode, output_mode, name_symbol, Rc::new(func));
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
    let func = Func::new_mutable(input_mode, output_mode, name_symbol, Rc::new(func));
    let func_val = FuncVal::from(func);
    Named::new(name, func_val)
}

#[allow(unused)]
fn id_mode() -> Mode {
    Mode {
        default: Transform::Id,
        specialized: None,
    }
}

fn form_mode() -> Mode {
    Mode {
        default: Transform::Form,
        specialized: None,
    }
}

#[allow(unused)]
fn eval_mode() -> Mode {
    Mode {
        default: Transform::Eval,
        specialized: None,
    }
}

#[allow(unused)]
fn pair_mode(first: Mode, second: Mode, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        pair: Pair::new(first, second),
        list: ListMode::All(default_mode.clone()),
        map: MapMode::All(Pair::new(default_mode.clone(), default_mode)),
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

#[allow(unused)]
fn list_mode(list_item: List<ListItemMode>, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        list: ListMode::Some(list_item),
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        map: MapMode::All(Pair::new(default_mode.clone(), default_mode)),
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

#[allow(unused)]
fn map_mode(map_mode: Map<Val, Mode>, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        map: MapMode::Some(map_mode),
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        list: ListMode::All(default_mode),
    };
    Mode {
        default,
        specialized: Some(Box::new(val_mode)),
    }
}

fn map_all_mode(key: Mode, value: Mode, default: Transform) -> Mode {
    let default_mode = Mode {
        default,
        specialized: None,
    };
    let val_mode = ValMode {
        map: MapMode::All(Pair::new(key, value)),
        pair: Pair::new(default_mode.clone(), default_mode.clone()),
        list: ListMode::All(default_mode),
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
