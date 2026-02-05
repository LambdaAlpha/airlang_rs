use derive_more::From;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::abort_by_bug_with_msg;
use crate::semantics::func::CtxCompFunc;
use crate::semantics::func::CtxFn;
use crate::semantics::func::CtxPrimFunc;
use crate::semantics::func::FreeCompFunc;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::val::Val;
use crate::type_::wrap::rc_wrap;

#[derive(Clone, PartialEq, Eq, From)]
pub enum FuncVal {
    FreePrim(FreePrimFuncVal),
    FreeComp(FreeCompFuncVal),
    CtxPrim(CtxPrimFuncVal),
    CtxComp(CtxCompFuncVal),
}

rc_wrap!(pub FreePrimFuncVal(FreePrimFunc));

rc_wrap!(pub FreeCompFuncVal(FreeCompFunc));

rc_wrap!(pub CtxPrimFuncVal(CtxPrimFunc));

rc_wrap!(pub CtxCompFuncVal(CtxCompFunc));

macro_rules! match_func_val {
    ($self:ident, $name:ident => $body:expr) => {
        match $self {
            FuncVal::FreePrim($name) => $body,
            FuncVal::FreeComp($name) => $body,
            FuncVal::CtxPrim($name) => $body,
            FuncVal::CtxComp($name) => $body,
        }
    };
}

impl FreeFn<Cfg, Val, Val> for FuncVal {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match self {
            FuncVal::FreePrim(f) => return f.free_call(cfg, input),
            FuncVal::FreeComp(f) => return f.free_call(cfg, input),
            _ => {},
        }
        let msg = format!("eval: no context for function {self}");
        abort_by_bug_with_msg(cfg, msg.into())
    }
}

impl CtxFn<Cfg, Val, Val, Val> for FuncVal {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match self {
            FuncVal::FreePrim(f) => f.free_call(cfg, input),
            FuncVal::FreeComp(f) => f.free_call(cfg, input),
            FuncVal::CtxPrim(f) => f.ctx_call(cfg, ctx, input),
            FuncVal::CtxComp(f) => f.ctx_call(cfg, ctx, input),
        }
    }
}

impl FuncVal {
    pub fn raw_input(&self) -> bool {
        match_func_val!(self, f => f.raw_input)
    }

    pub fn prelude(&self) -> Option<&Val> {
        match self {
            FuncVal::FreePrim(_) => None,
            FuncVal::FreeComp(f) => Some(&f.comp.prelude),
            FuncVal::CtxPrim(_) => None,
            FuncVal::CtxComp(f) => Some(&f.comp.prelude),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            FuncVal::FreePrim(_) => true,
            FuncVal::FreeComp(_) => false,
            FuncVal::CtxPrim(_) => true,
            FuncVal::CtxComp(_) => false,
        }
    }

    pub fn is_free(&self) -> bool {
        match self {
            FuncVal::FreePrim(_) => true,
            FuncVal::FreeComp(_) => true,
            FuncVal::CtxPrim(_) => false,
            FuncVal::CtxComp(_) => false,
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            FuncVal::FreePrim(_) => true,
            FuncVal::FreeComp(_) => true,
            FuncVal::CtxPrim(f) => f.const_,
            FuncVal::CtxComp(f) => f.comp.const_,
        }
    }
}

impl Default for FreePrimFuncVal {
    fn default() -> Self {
        Self::from(FreePrimFunc::default())
    }
}

impl Default for FuncVal {
    fn default() -> Self {
        Self::FreePrim(FreePrimFuncVal::default())
    }
}
