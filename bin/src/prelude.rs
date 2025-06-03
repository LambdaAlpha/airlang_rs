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

pub(crate) struct FreeFn<F> {
    pub(crate) id: &'static str,
    pub(crate) f: F,
    pub(crate) mode: FuncMode,
}

pub(crate) struct DynFn<F> {
    pub(crate) id: &'static str,
    pub(crate) f: F,
    pub(crate) mode: FuncMode,
    pub(crate) ctx_explicit: bool,
}

impl<F: FreeCellFnExt + 'static> FreeFn<F> {
    #[expect(dead_code)]
    pub(crate) fn free_cell(self) -> FreeCellPrimFuncVal {
        let id = unsafe { Symbol::from_str_unchecked(self.id) };
        let func = FreeCellPrimFunc::new(id, Box::new(self.f), self.mode);
        FreeCellPrimFuncVal::from(func)
    }
}

impl<F: FreeStaticFn<Val, Val> + 'static> FreeFn<F> {
    pub(crate) fn free_static(self) -> FreeStaticPrimFuncVal {
        let id = unsafe { Symbol::from_str_unchecked(self.id) };
        let func = FreeStaticPrimFunc::new(id, Rc::new(self.f), self.mode);
        FreeStaticPrimFuncVal::from(func)
    }
}

impl<F: ConstCellFnExt + 'static> DynFn<F> {
    #[expect(dead_code)]
    pub(crate) fn const_cell(self) -> ConstCellPrimFuncVal {
        let id = unsafe { Symbol::from_str_unchecked(self.id) };
        let func = ConstCellPrimFunc::new(id, Box::new(self.f), self.mode, self.ctx_explicit);
        ConstCellPrimFuncVal::from(func)
    }
}

impl<F: ConstStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    #[expect(dead_code)]
    pub(crate) fn const_static(self) -> ConstStaticPrimFuncVal {
        let id = unsafe { Symbol::from_str_unchecked(self.id) };
        let func = ConstStaticPrimFunc::new(id, Rc::new(self.f), self.mode, self.ctx_explicit);
        ConstStaticPrimFuncVal::from(func)
    }
}

impl<F: MutCellFnExt + 'static> DynFn<F> {
    #[expect(dead_code)]
    pub(crate) fn mut_cell(self) -> MutCellPrimFuncVal {
        let id = unsafe { Symbol::from_str_unchecked(self.id) };
        let func = MutCellPrimFunc::new(id, Box::new(self.f), self.mode, self.ctx_explicit);
        MutCellPrimFuncVal::from(func)
    }
}

impl<F: MutStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    pub(crate) fn mut_static(self) -> MutStaticPrimFuncVal {
        let id = unsafe { Symbol::from_str_unchecked(self.id) };
        let func = MutStaticPrimFunc::new(id, Rc::new(self.f), self.mode, self.ctx_explicit);
        MutStaticPrimFuncVal::from(func)
    }
}

fn free_impl(func: fn(Val) -> Val) -> FreeStaticImpl<Val, Val> {
    FreeStaticImpl::new(func)
}

#[allow(dead_code)]
fn const_impl(func: fn(ConstRef<Val>, Val) -> Val) -> ConstStaticImpl<Val, Val, Val> {
    ConstStaticImpl::new(FreeStaticImpl::default, func)
}

fn mut_impl(func: fn(&mut Val, Val) -> Val) -> MutStaticImpl<Val, Val, Val> {
    MutStaticImpl::new(FreeStaticImpl::default, ConstStaticImpl::default, func)
}

mod repl;

mod eval;

mod process;
