use std::rc::Rc;

use airlang::{
    InvariantTag,
    MutableCtx,
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

mod repl;

mod eval;

mod process;
