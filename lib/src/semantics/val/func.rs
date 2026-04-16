use std::rc::Rc;

use derive_more::Deref;
use derive_more::From;

use crate::semantics::cfg::Cfg;
use crate::semantics::func::CompFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimFunc;
use crate::semantics::func::PrimInput;
use crate::semantics::val::Val;
use crate::type_::wrap::rc_wrap;
use crate::utils::memory::leak_const;

#[derive(Clone, PartialEq, Eq, From)]
pub enum FuncVal {
    Prim(PrimFuncVal),
    Comp(CompFuncVal),
}

#[derive(Copy, Clone, Deref)]
pub struct PrimFuncVal(&'static PrimFunc);

impl From<PrimFunc> for PrimFuncVal {
    fn from(val: PrimFunc) -> Self {
        Self(leak_const(val))
    }
}

impl PartialEq for PrimFuncVal {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl Eq for PrimFuncVal {}

rc_wrap!(pub CompFuncVal(CompFunc));

impl PartialEq for CompFuncVal {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for CompFuncVal {}

impl DynFunc<Cfg, Val, Val, Val> for FuncVal {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match self {
            FuncVal::Prim(prim) => prim.call(cfg, ctx, input),
            FuncVal::Comp(comp) => comp.call(cfg, ctx, input),
        }
    }
}

impl FuncVal {
    pub fn input(&self) -> PrimInput {
        match self {
            FuncVal::Prim(f) => f.input,
            FuncVal::Comp(f) => f.input.to_prim_input(),
        }
    }

    pub fn ctx(&self) -> PrimCtx {
        match self {
            FuncVal::Prim(f) => f.ctx,
            FuncVal::Comp(f) => f.ctx.to_prim_ctx(),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            FuncVal::Prim(_) => true,
            FuncVal::Comp(_) => false,
        }
    }

    pub fn prelude(&self) -> Option<&Val> {
        match self {
            FuncVal::Prim(_) => None,
            FuncVal::Comp(f) => Some(&f.prelude),
        }
    }

    pub fn id(&self) -> usize {
        match self {
            FuncVal::Prim(f) => (f.0 as *const PrimFunc).addr(),
            FuncVal::Comp(f) => Rc::as_ptr(&f.0).addr(),
        }
    }
}
