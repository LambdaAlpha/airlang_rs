use std::fmt::Debug;
use std::fmt::Formatter;

use derive_more::From;

use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstCompFunc;
use crate::semantics::func::ConstFn;
use crate::semantics::func::ConstPrimFunc;
use crate::semantics::func::FreeCompFunc;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FreePrimFunc;
use crate::semantics::func::MutCompFunc;
use crate::semantics::func::MutFn;
use crate::semantics::func::MutPrimFunc;
use crate::semantics::memo::Memo;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Key;
use crate::type_::wrap::rc_wrap;

#[derive(Clone, PartialEq, Eq, From)]
pub enum FuncVal {
    FreePrim(FreePrimFuncVal),
    FreeComp(FreeCompFuncVal),
    ConstPrim(ConstPrimFuncVal),
    ConstComp(ConstCompFuncVal),
    MutPrim(MutPrimFuncVal),
    MutComp(MutCompFuncVal),
}

rc_wrap!(pub FreePrimFuncVal(FreePrimFunc));

rc_wrap!(pub FreeCompFuncVal(FreeCompFunc));

rc_wrap!(pub ConstPrimFuncVal(ConstPrimFunc));

rc_wrap!(pub ConstCompFuncVal(ConstCompFunc));

rc_wrap!(pub MutPrimFuncVal(MutPrimFunc));

rc_wrap!(pub MutCompFuncVal(MutCompFunc));

macro_rules! match_func_val {
    ($self:ident, $name:ident => $body:expr) => {
        match $self {
            FuncVal::FreePrim($name) => $body,
            FuncVal::FreeComp($name) => $body,
            FuncVal::ConstPrim($name) => $body,
            FuncVal::ConstComp($name) => $body,
            FuncVal::MutPrim($name) => $body,
            FuncVal::MutComp($name) => $body,
        }
    };
}

impl FreeFn<Cfg, Val, Val> for FuncVal {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match_func_val!(self, f => f.free_call(cfg,input))
    }
}

impl ConstFn<Cfg, Val, Val, Val> for FuncVal {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            FuncVal::FreePrim(f) => f.free_call(cfg, input),
            FuncVal::FreeComp(f) => f.free_call(cfg, input),
            FuncVal::ConstPrim(f) => f.const_call(cfg, ctx, input),
            FuncVal::ConstComp(f) => f.const_call(cfg, ctx, input),
            FuncVal::MutPrim(f) => f.const_call(cfg, ctx, input),
            FuncVal::MutComp(f) => f.const_call(cfg, ctx, input),
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for FuncVal {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match self {
            FuncVal::FreePrim(f) => f.free_call(cfg, input),
            FuncVal::FreeComp(f) => f.free_call(cfg, input),
            FuncVal::ConstPrim(f) => f.const_call(cfg, ConstRef::new(ctx), input),
            FuncVal::ConstComp(f) => f.const_call(cfg, ConstRef::new(ctx), input),
            FuncVal::MutPrim(f) => f.mut_call(cfg, ctx, input),
            FuncVal::MutComp(f) => f.mut_call(cfg, ctx, input),
        }
    }
}

impl FuncVal {
    pub fn id(&self) -> Key {
        match_func_val!(self, f => f.id.clone())
    }

    pub fn raw_input(&self) -> bool {
        match_func_val!(self, f => f.raw_input)
    }

    pub fn memo(&self) -> Option<&Memo> {
        match self {
            FuncVal::FreePrim(_) => None,
            FuncVal::FreeComp(f) => Some(&f.memo),
            FuncVal::ConstPrim(_) => None,
            FuncVal::ConstComp(f) => Some(&f.memo),
            FuncVal::MutPrim(_) => None,
            FuncVal::MutComp(f) => Some(&f.memo),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            FuncVal::FreePrim(_) => true,
            FuncVal::FreeComp(_) => false,
            FuncVal::ConstPrim(_) => true,
            FuncVal::ConstComp(_) => false,
            FuncVal::MutPrim(_) => true,
            FuncVal::MutComp(_) => false,
        }
    }

    pub fn ctx_access(&self) -> CtxAccess {
        match self {
            FuncVal::FreePrim(_) => CtxAccess::Free,
            FuncVal::FreeComp(_) => CtxAccess::Free,
            FuncVal::ConstPrim(_) => CtxAccess::Const,
            FuncVal::ConstComp(_) => CtxAccess::Const,
            FuncVal::MutPrim(_) => CtxAccess::Mut,
            FuncVal::MutComp(_) => CtxAccess::Mut,
        }
    }
}

impl Debug for FuncVal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match_func_val!(self, f => f.fmt(formatter))
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
