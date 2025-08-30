use std::rc::Rc;

use super::mode::FuncMode;
use super::mode::Mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::ConstPrimFunc;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::func::MutFn;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::func::Setup;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Symbol;

thread_local!(pub(in super) static CORE_SETUP: CoreSetup = CoreSetup::default());

#[derive(Default, Clone)]
pub struct CoreSetup {}

pub struct FreeSetupFn<F> {
    pub id: &'static str,
    pub f: F,
}

pub struct DynSetupFn<F> {
    pub id: &'static str,
    pub f: F,
}

impl<F: FreeFn<Cfg, Val, Val> + 'static> FreeSetupFn<F> {
    pub fn free(self) -> FreePrimFuncVal {
        let func = FreePrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Setup::none(),
        };
        FreePrimFuncVal::from(func)
    }
}

impl<F: ConstFn<Cfg, Val, Val, Val> + 'static> DynSetupFn<F> {
    pub fn const_(self) -> ConstPrimFuncVal {
        let func = ConstPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Setup::none(),
        };
        ConstPrimFuncVal::from(func)
    }
}

impl<F: MutFn<Cfg, Val, Val, Val> + 'static> DynSetupFn<F> {
    pub fn mut_(self) -> MutPrimFuncVal {
        let func = MutPrimFunc {
            id: Symbol::from_str_unchecked(self.id),
            fn_: Rc::new(self.f),
            setup: Setup::none(),
        };
        MutPrimFuncVal::from(func)
    }
}

pub fn free_mode(mode: Mode) -> FuncMode {
    FuncMode { call: mode, solve: FuncMode::default_mode() }
}

pub fn default_free_mode() -> FuncMode {
    free_mode(FuncMode::default_mode())
}

pub fn dyn_mode(mode: Mode) -> FuncMode {
    FuncMode { call: mode, solve: FuncMode::default_mode() }
}

pub fn default_dyn_mode() -> FuncMode {
    dyn_mode(FuncMode::default_mode())
}
