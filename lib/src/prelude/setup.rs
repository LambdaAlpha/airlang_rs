use std::rc::Rc;

use super::mode::FuncMode;
use super::mode::Mode;
use super::mode::SymbolMode;
use super::setup::ctx::CtxSetup;
use crate::semantics::func::ConstCellFnVal;
use crate::semantics::func::ConstCellPrimFunc;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::ConstStaticPrimFunc;
use crate::semantics::func::FreeCellFnVal;
use crate::semantics::func::FreeCellPrimFunc;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FreeStaticPrimFunc;
use crate::semantics::func::MutCellFnVal;
use crate::semantics::func::MutCellPrimFunc;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::MutStaticPrimFunc;
use crate::semantics::val::ConstCellPrimFuncVal;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeCellPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutCellPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Symbol;

thread_local!(pub(in super) static CORE_SETUP: CoreSetup = CoreSetup::default());

#[derive(Default, Clone)]
pub struct CoreSetup {
    pub ctx: CtxSetup,
}

pub struct FreeFn<F> {
    pub id: &'static str,
    pub f: F,
}

pub struct DynFn<F> {
    pub id: &'static str,
    pub f: F,
}

impl<F: FreeCellFnVal + 'static> FreeFn<F> {
    pub fn free_cell(self) -> FreeCellPrimFuncVal {
        let func = FreeCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            setup: None,
        };
        FreeCellPrimFuncVal::from(func)
    }
}

impl<F: FreeStaticFn<Val, Val> + 'static> FreeFn<F> {
    pub fn free_static(self) -> FreeStaticPrimFuncVal {
        let func = FreeStaticPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: None,
        };
        FreeStaticPrimFuncVal::from(func)
    }
}

impl<F: ConstCellFnVal + 'static> DynFn<F> {
    pub fn const_cell(self) -> ConstCellPrimFuncVal {
        let func = ConstCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            setup: None,
            ctx_explicit: false,
        };
        ConstCellPrimFuncVal::from(func)
    }
}

impl<F: ConstStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    pub fn const_static(self) -> ConstStaticPrimFuncVal {
        let func = ConstStaticPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: None,
            ctx_explicit: false,
        };
        ConstStaticPrimFuncVal::from(func)
    }
}

impl<F: MutCellFnVal + 'static> DynFn<F> {
    pub fn mut_cell(self) -> MutCellPrimFuncVal {
        let func = MutCellPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Box::new(self.f),
            setup: None,
            ctx_explicit: false,
        };
        MutCellPrimFuncVal::from(func)
    }
}

impl<F: MutStaticFn<Val, Val, Val> + 'static> DynFn<F> {
    pub fn mut_static(self) -> MutStaticPrimFuncVal {
        let func = MutStaticPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: None,
            ctx_explicit: false,
        };
        MutStaticPrimFuncVal::from(func)
    }
}

pub fn ctx_default_mode() -> Option<Mode> {
    FuncMode::pair_mode(FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

pub fn ref_mode() -> Option<Mode> {
    let ref_ = CORE_SETUP.with(|p| p.ctx.ref_.clone());
    Some(Mode::Func(ref_.into()))
}

mod ctx;
