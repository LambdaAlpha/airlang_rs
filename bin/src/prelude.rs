use std::rc::Rc;

use airlang::ConstCellFnExt;
use airlang::ConstCellPrimFunc;
use airlang::ConstCellPrimFuncVal;
use airlang::ConstRef;
use airlang::ConstStaticFn;
use airlang::ConstStaticImpl;
use airlang::ConstStaticPrimFunc;
use airlang::ConstStaticPrimFuncVal;
use airlang::FreeCellFnExt;
use airlang::FreeCellPrimFunc;
use airlang::FreeCellPrimFuncVal;
use airlang::FreeStaticFn;
use airlang::FreeStaticImpl;
use airlang::FreeStaticPrimFunc;
use airlang::FreeStaticPrimFuncVal;
use airlang::FuncMode;
use airlang::FuncVal;
use airlang::MutCellFnExt;
use airlang::MutCellPrimFunc;
use airlang::MutCellPrimFuncVal;
use airlang::MutStaticFn;
use airlang::MutStaticImpl;
use airlang::MutStaticPrimFunc;
use airlang::MutStaticPrimFuncVal;
use airlang::Prelude;
use airlang::PreludeCtx;
use airlang::Symbol;
use airlang::Val;

use crate::prelude::eval::EvalPrelude;
use crate::prelude::process::ProcessPrelude;
use crate::prelude::repl::ReplPrelude;

#[derive(Default)]
pub(crate) struct AllPrelude {
    pub(crate) repl: ReplPrelude,
    pub(crate) eval: EvalPrelude,
    pub(crate) process: ProcessPrelude,
}

impl Prelude for AllPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
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

fn free_cell_fn(name: &'static str, func: impl FreeCellFnExt + 'static, mode: FuncMode) -> FuncVal {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = FreeCellPrimFunc::new(id, fn1, mode);
    let func_val = FreeCellPrimFuncVal::from(func);
    FuncVal::FreeCellPrim(func_val)
}

#[expect(dead_code)]
fn named_free_cell_fn(
    name: &'static str, func: impl FreeCellFnExt + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = free_cell_fn(name, func, mode);
    Named::new(name, f)
}

fn const_cell_fn(
    name: &'static str, func: impl ConstCellFnExt + 'static, mode: FuncMode, ctx_explicit: bool,
) -> FuncVal {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = ConstCellPrimFunc::new(id, fn1, mode, ctx_explicit);
    let func_val = ConstCellPrimFuncVal::from(func);
    FuncVal::ConstCellPrim(func_val)
}

#[expect(dead_code)]
fn named_const_cell_fn(
    name: &'static str, func: impl ConstCellFnExt + 'static, mode: FuncMode, ctx_explicit: bool,
) -> Named<FuncVal> {
    let f = const_cell_fn(name, func, mode, ctx_explicit);
    Named::new(name, f)
}

fn mut_cell_fn(
    name: &'static str, func: impl MutCellFnExt + 'static, mode: FuncMode, ctx_explicit: bool,
) -> FuncVal {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Box::new(func);
    let func = MutCellPrimFunc::new(id, fn1, mode, ctx_explicit);
    let func_val = MutCellPrimFuncVal::from(func);
    FuncVal::MutCellPrim(func_val)
}

#[expect(dead_code)]
fn named_mut_cell_fn(
    name: &'static str, func: impl MutCellFnExt + 'static, mode: FuncMode, ctx_explicit: bool,
) -> Named<FuncVal> {
    let f = mut_cell_fn(name, func, mode, ctx_explicit);
    Named::new(name, f)
}

fn free_impl(func: fn(Val) -> Val) -> FreeStaticImpl<Val, Val> {
    FreeStaticImpl::new(func)
}

fn free_fn(
    name: &'static str, func: impl FreeStaticFn<Val, Val> + 'static, mode: FuncMode,
) -> FuncVal {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = FreeStaticPrimFunc::new(id, fn1, mode);
    let func_val = FreeStaticPrimFuncVal::from(func);
    FuncVal::FreeStaticPrim(func_val)
}

fn named_free_fn(
    name: &'static str, func: impl FreeStaticFn<Val, Val> + 'static, mode: FuncMode,
) -> Named<FuncVal> {
    let f = free_fn(name, func, mode);
    Named::new(name, f)
}

#[allow(dead_code)]
fn const_impl(func: fn(ConstRef<Val>, Val) -> Val) -> ConstStaticImpl<Val, Val, Val> {
    ConstStaticImpl::new(FreeStaticImpl::default, func)
}

fn const_fn(
    name: &'static str, func: impl ConstStaticFn<Val, Val, Val> + 'static, mode: FuncMode,
    ctx_explicit: bool,
) -> FuncVal {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = ConstStaticPrimFunc::new(id, fn1, mode, ctx_explicit);
    let func_val = ConstStaticPrimFuncVal::from(func);
    FuncVal::ConstStaticPrim(func_val)
}

#[allow(dead_code)]
fn named_const_fn(
    name: &'static str, func: impl ConstStaticFn<Val, Val, Val> + 'static, mode: FuncMode,
    ctx_explicit: bool,
) -> Named<FuncVal> {
    let f = const_fn(name, func, mode, ctx_explicit);
    Named::new(name, f)
}

fn mut_impl(func: fn(&mut Val, Val) -> Val) -> MutStaticImpl<Val, Val, Val> {
    MutStaticImpl::new(FreeStaticImpl::default, ConstStaticImpl::default, func)
}

fn mut_fn(
    name: &'static str, func: impl MutStaticFn<Val, Val, Val> + 'static, mode: FuncMode,
    ctx_explicit: bool,
) -> FuncVal {
    let id = unsafe { Symbol::from_str_unchecked(name) };
    let fn1 = Rc::new(func);
    let func = MutStaticPrimFunc::new(id, fn1, mode, ctx_explicit);
    let func_val = MutStaticPrimFuncVal::from(func);
    FuncVal::MutStaticPrim(func_val)
}

fn named_mut_fn(
    name: &'static str, func: impl MutStaticFn<Val, Val, Val> + 'static, mode: FuncMode,
    ctx_explicit: bool,
) -> Named<FuncVal> {
    let f = mut_fn(name, func, mode, ctx_explicit);
    Named::new(name, f)
}

mod repl;

mod eval;

mod process;
