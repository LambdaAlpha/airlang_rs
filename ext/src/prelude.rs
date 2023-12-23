use std::rc::Rc;

use airlang::{
    InvariantTag,
    MutableCtx,
    Symbol,
    Val,
    ValExt,
};

use crate::{
    func::ExtFuncVal,
    prelude::{
        file::FilePrelude,
        io::IoPrelude,
        process::ProcessPrelude,
    },
    ExtFunc,
};

thread_local!(pub(crate) static PRELUDE: AllPrelude = AllPrelude::default());

#[derive(Default)]
pub(crate) struct AllPrelude {
    pub(crate) io: IoPrelude,
    pub(crate) file: FilePrelude,
    pub(crate) process: ProcessPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, mut ctx: MutableCtx) {
        self.io.put(ctx.reborrow());
        self.file.put(ctx.reborrow());
        self.process.put(ctx.reborrow());
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

#[allow(unused)]
impl<T: ValExt + 'static> Named<T> {
    pub(crate) fn put(self, mut ctx: MutableCtx) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        let val = Val::Ext(Box::new(self.value));
        let _ = ctx.put(name, InvariantTag::Const, val);
    }
}

pub(crate) fn put_func(func: &Rc<ExtFunc>, mut ctx: MutableCtx) {
    let id = func.id.clone();
    let val = Val::Ext(Box::new(ExtFuncVal::from(func.clone())));
    let _ = ctx.put(id, InvariantTag::Const, val);
}

pub(crate) mod io;

pub(crate) mod file;

pub(crate) mod process;
