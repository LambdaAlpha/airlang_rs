use derive_more::From;

use crate::semantics::cfg::Cfg;
use crate::semantics::func::CompFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::func::PrimCtx;
use crate::semantics::func::PrimFunc;
use crate::semantics::func::PrimInput;
use crate::semantics::val::Val;
use crate::type_::wrap::rc_wrap;

#[derive(Clone, PartialEq, Eq, From)]
pub enum FuncVal {
    Prim(PrimFuncVal),
    Comp(CompFuncVal),
}

rc_wrap!(pub PrimFuncVal(PrimFunc));

rc_wrap!(pub CompFuncVal(CompFunc));

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
}

impl Default for PrimFuncVal {
    fn default() -> Self {
        Self::from(PrimFunc::default())
    }
}

impl Default for FuncVal {
    fn default() -> Self {
        Self::Prim(PrimFuncVal::default())
    }
}
