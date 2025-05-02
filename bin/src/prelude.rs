use std::rc::Rc;

use airlang::{
    ConstCellFnExt,
    ConstCellPrimFunc,
    ConstCellPrimFuncVal,
    ConstStaticFn,
    ConstStaticPrimFunc,
    ConstStaticPrimFuncVal,
    FreeCellFnExt,
    FreeCellPrimFunc,
    FreeCellPrimFuncVal,
    FreeStaticFn,
    FreeStaticPrimFunc,
    FreeStaticPrimFuncVal,
    FuncMode,
    FuncVal,
    MutCellFnExt,
    MutCellPrimFunc,
    MutCellPrimFuncVal,
    MutStaticFn,
    MutStaticPrimFunc,
    MutStaticPrimFuncVal,
    Prelude,
    PreludeCtx,
    Symbol,
    Val,
};
use airlang_ext::ExtPrelude;

use crate::prelude::{
    eval::EvalPrelude,
    process::ProcessPrelude,
    repl::ReplPrelude,
};

#[derive(Default)]
pub(crate) struct AllPrelude {
    pub(crate) ext: ExtPrelude,
    pub(crate) repl: ReplPrelude,
    pub(crate) eval: EvalPrelude,
    pub(crate) process: ProcessPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.ext.put(ctx);
        self.repl.put(ctx);
        self.eval.put(ctx);
        self.process.put(ctx);
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
    pub(crate) fn put(&self, ctx: &mut dyn PreludeCtx) {
        let name = unsafe { Symbol::from_str_unchecked(self.name) };
        let val = self.value.clone().into();
        ctx.put(name, val);
    }
}

#[expect(dead_code)]
fn named_free_cell_fn(
    name: &'static str, func: impl FreeCellFnExt + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = FreeCellPrimFunc::new_extension(id, fn1, mode);
    let func_val = FreeCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::FreeCellPrim(func_val))
}

#[expect(dead_code)]
fn named_const_cell_fn(
    name: &'static str, func: impl ConstCellFnExt + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = ConstCellPrimFunc::new_extension(id, fn1, mode);
    let func_val = ConstCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::ConstCellPrim(func_val))
}

#[expect(dead_code)]
fn named_mut_cell_fn(
    name: &'static str, func: impl MutCellFnExt + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = MutCellPrimFunc::new_extension(id, fn1, mode);
    let func_val = MutCellPrimFuncVal::from(func);
    Named::new(name, FuncVal::MutCellPrim(func_val))
}

fn named_free_fn(
    name: &'static str, func: impl FreeStaticFn + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = FreeStaticPrimFunc::new_extension(id, fn1, mode);
    let func_val = FreeStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::FreeStaticPrim(func_val))
}

#[expect(dead_code)]
fn named_const_fn(
    name: &'static str, func: impl ConstStaticFn + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = ConstStaticPrimFunc::new_extension(id, fn1, mode);
    let func_val = ConstStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::ConstStaticPrim(func_val))
}

fn named_mut_fn(
    name: &'static str, func: impl MutStaticFn + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = MutStaticPrimFunc::new_extension(id, fn1, mode);
    let func_val = MutStaticPrimFuncVal::from(func);
    Named::new(name, FuncVal::MutStaticPrim(func_val))
}

mod repl;

mod eval;

mod process;
