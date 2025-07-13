use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;

use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstCellCompFunc;
use crate::semantics::func::ConstCellFn;
use crate::semantics::func::ConstCellPrimFunc;
use crate::semantics::func::ConstStaticCompFunc;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::ConstStaticPrimFunc;
use crate::semantics::func::FreeCellCompFunc;
use crate::semantics::func::FreeCellFn;
use crate::semantics::func::FreeCellPrimFunc;
use crate::semantics::func::FreeStaticCompFunc;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FreeStaticPrimFunc;
use crate::semantics::func::FuncSetup;
use crate::semantics::func::MutCellCompFunc;
use crate::semantics::func::MutCellFn;
use crate::semantics::func::MutCellPrimFunc;
use crate::semantics::func::MutStaticCompFunc;
use crate::semantics::func::MutStaticFn;
use crate::semantics::func::MutStaticPrimFunc;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;
use crate::type_::wrap::box_wrap;
use crate::type_::wrap::rc_wrap;

// todo impl derive from
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FuncVal {
    FreeCellPrim(FreeCellPrimFuncVal),
    FreeCellComp(FreeCellCompFuncVal),
    FreeStaticPrim(FreeStaticPrimFuncVal),
    FreeStaticComp(FreeStaticCompFuncVal),
    ConstCellPrim(ConstCellPrimFuncVal),
    ConstCellComp(ConstCellCompFuncVal),
    ConstStaticPrim(ConstStaticPrimFuncVal),
    ConstStaticComp(ConstStaticCompFuncVal),
    MutCellPrim(MutCellPrimFuncVal),
    MutCellComp(MutCellCompFuncVal),
    MutStaticPrim(MutStaticPrimFuncVal),
    MutStaticComp(MutStaticCompFuncVal),
}

box_wrap!(pub FreeCellPrimFuncVal(FreeCellPrimFunc));

box_wrap!(pub FreeCellCompFuncVal(FreeCellCompFunc));

rc_wrap!(pub FreeStaticPrimFuncVal(FreeStaticPrimFunc));

rc_wrap!(pub FreeStaticCompFuncVal(FreeStaticCompFunc));

box_wrap!(pub ConstCellPrimFuncVal(ConstCellPrimFunc));

box_wrap!(pub ConstCellCompFuncVal(ConstCellCompFunc));

rc_wrap!(pub ConstStaticPrimFuncVal(ConstStaticPrimFunc));

rc_wrap!(pub ConstStaticCompFuncVal(ConstStaticCompFunc));

box_wrap!(pub MutCellPrimFuncVal(MutCellPrimFunc));

box_wrap!(pub MutCellCompFuncVal(MutCellCompFunc));

rc_wrap!(pub MutStaticPrimFuncVal(MutStaticPrimFunc));

rc_wrap!(pub MutStaticCompFuncVal(MutStaticCompFunc));

macro_rules! match_func_val {
    ($self:ident, $name:ident => $body:expr) => {
        match $self {
            FuncVal::FreeCellPrim($name) => $body,
            FuncVal::FreeCellComp($name) => $body,
            FuncVal::FreeStaticPrim($name) => $body,
            FuncVal::FreeStaticComp($name) => $body,
            FuncVal::ConstCellPrim($name) => $body,
            FuncVal::ConstCellComp($name) => $body,
            FuncVal::ConstStaticPrim($name) => $body,
            FuncVal::ConstStaticComp($name) => $body,
            FuncVal::MutCellPrim($name) => $body,
            FuncVal::MutCellComp($name) => $body,
            FuncVal::MutStaticPrim($name) => $body,
            FuncVal::MutStaticComp($name) => $body,
        }
    };
}

impl FreeStaticFn<Val, Val> for FuncVal {
    fn free_static_call(&self, input: Val) -> Val {
        match self {
            FuncVal::FreeCellPrim(f) => f.free_static_call(input),
            FuncVal::FreeCellComp(f) => f.free_static_call(input),
            FuncVal::FreeStaticPrim(f) => f.free_static_call(input),
            FuncVal::FreeStaticComp(f) => f.free_static_call(input),
            FuncVal::ConstCellPrim(f) => f.free_static_call(input),
            FuncVal::ConstCellComp(f) => f.free_static_call(input),
            FuncVal::ConstStaticPrim(f) => f.free_static_call(input),
            FuncVal::ConstStaticComp(f) => f.free_static_call(input),
            FuncVal::MutCellPrim(f) => f.free_static_call(input),
            FuncVal::MutCellComp(f) => f.free_static_call(input),
            FuncVal::MutStaticPrim(f) => f.free_static_call(input),
            FuncVal::MutStaticComp(f) => f.free_static_call(input),
        }
    }
}

impl FreeCellFn<Val, Val> for FuncVal {
    fn free_cell_call(&mut self, input: Val) -> Val {
        match self {
            FuncVal::FreeCellPrim(f) => f.free_cell_call(input),
            FuncVal::FreeCellComp(f) => f.free_cell_call(input),
            FuncVal::FreeStaticPrim(f) => f.free_static_call(input),
            FuncVal::FreeStaticComp(f) => f.free_static_call(input),
            FuncVal::ConstCellPrim(f) => f.free_cell_call(input),
            FuncVal::ConstCellComp(f) => f.free_cell_call(input),
            FuncVal::ConstStaticPrim(f) => f.free_static_call(input),
            FuncVal::ConstStaticComp(f) => f.free_static_call(input),
            FuncVal::MutCellPrim(f) => f.free_cell_call(input),
            FuncVal::MutCellComp(f) => f.free_cell_call(input),
            FuncVal::MutStaticPrim(f) => f.free_static_call(input),
            FuncVal::MutStaticComp(f) => f.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Val, Val, Val> for FuncVal {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            FuncVal::FreeCellPrim(f) => f.free_static_call(input),
            FuncVal::FreeCellComp(f) => f.free_static_call(input),
            FuncVal::FreeStaticPrim(f) => f.free_static_call(input),
            FuncVal::FreeStaticComp(f) => f.free_static_call(input),
            FuncVal::ConstCellPrim(f) => f.const_static_call(ctx, input),
            FuncVal::ConstCellComp(f) => f.const_static_call(ctx, input),
            FuncVal::ConstStaticPrim(f) => f.const_static_call(ctx, input),
            FuncVal::ConstStaticComp(f) => f.const_static_call(ctx, input),
            FuncVal::MutCellPrim(f) => f.const_static_call(ctx, input),
            FuncVal::MutCellComp(f) => f.const_static_call(ctx, input),
            FuncVal::MutStaticPrim(f) => f.const_static_call(ctx, input),
            FuncVal::MutStaticComp(f) => f.const_static_call(ctx, input),
        }
    }
}

impl ConstCellFn<Val, Val, Val> for FuncVal {
    fn const_cell_call(&mut self, ctx: ConstRef<Val>, input: Val) -> Val {
        match self {
            FuncVal::FreeCellPrim(f) => f.free_cell_call(input),
            FuncVal::FreeCellComp(f) => f.free_cell_call(input),
            FuncVal::FreeStaticPrim(f) => f.free_static_call(input),
            FuncVal::FreeStaticComp(f) => f.free_static_call(input),
            FuncVal::ConstCellPrim(f) => f.const_cell_call(ctx, input),
            FuncVal::ConstCellComp(f) => f.const_cell_call(ctx, input),
            FuncVal::ConstStaticPrim(f) => f.const_static_call(ctx, input),
            FuncVal::ConstStaticComp(f) => f.const_static_call(ctx, input),
            FuncVal::MutCellPrim(f) => f.const_cell_call(ctx, input),
            FuncVal::MutCellComp(f) => f.const_cell_call(ctx, input),
            FuncVal::MutStaticPrim(f) => f.const_static_call(ctx, input),
            FuncVal::MutStaticComp(f) => f.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, Val, Val> for FuncVal {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        match self {
            FuncVal::FreeCellPrim(f) => f.free_static_call(input),
            FuncVal::FreeCellComp(f) => f.free_static_call(input),
            FuncVal::FreeStaticPrim(f) => f.free_static_call(input),
            FuncVal::FreeStaticComp(f) => f.free_static_call(input),
            FuncVal::ConstCellPrim(f) => f.const_static_call(ConstRef::new(ctx), input),
            FuncVal::ConstCellComp(f) => f.const_static_call(ConstRef::new(ctx), input),
            FuncVal::ConstStaticPrim(f) => f.const_static_call(ConstRef::new(ctx), input),
            FuncVal::ConstStaticComp(f) => f.const_static_call(ConstRef::new(ctx), input),
            FuncVal::MutCellPrim(f) => f.mut_static_call(ctx, input),
            FuncVal::MutCellComp(f) => f.mut_static_call(ctx, input),
            FuncVal::MutStaticPrim(f) => f.mut_static_call(ctx, input),
            FuncVal::MutStaticComp(f) => f.mut_static_call(ctx, input),
        }
    }
}

impl MutCellFn<Val, Val, Val> for FuncVal {
    fn mut_cell_call(&mut self, ctx: &mut Val, input: Val) -> Val {
        match self {
            FuncVal::FreeCellPrim(f) => f.free_cell_call(input),
            FuncVal::FreeCellComp(f) => f.free_cell_call(input),
            FuncVal::FreeStaticPrim(f) => f.free_static_call(input),
            FuncVal::FreeStaticComp(f) => f.free_static_call(input),
            FuncVal::ConstCellPrim(f) => f.const_cell_call(ConstRef::new(ctx), input),
            FuncVal::ConstCellComp(f) => f.const_cell_call(ConstRef::new(ctx), input),
            FuncVal::ConstStaticPrim(f) => f.const_static_call(ConstRef::new(ctx), input),
            FuncVal::ConstStaticComp(f) => f.const_static_call(ConstRef::new(ctx), input),
            FuncVal::MutCellPrim(f) => f.mut_cell_call(ctx, input),
            FuncVal::MutCellComp(f) => f.mut_cell_call(ctx, input),
            FuncVal::MutStaticPrim(f) => f.mut_static_call(ctx, input),
            FuncVal::MutStaticComp(f) => f.mut_static_call(ctx, input),
        }
    }
}

impl FuncSetup for FuncVal {
    fn forward_ctx(&self) -> Option<&FuncVal> {
        match_func_val!(self, f => f.forward_ctx())
    }

    fn forward_input(&self) -> Option<&FuncVal> {
        match_func_val!(self, f => f.forward_input())
    }

    fn reverse_ctx(&self) -> Option<&FuncVal> {
        match_func_val!(self, f => f.reverse_ctx())
    }

    fn reverse_input(&self) -> Option<&FuncVal> {
        match_func_val!(self, f => f.reverse_input())
    }
}

impl FuncVal {
    pub fn id(&self) -> Option<Symbol> {
        match self {
            FuncVal::FreeCellPrim(f) => Some(f.id.clone()),
            FuncVal::FreeCellComp(_) => None,
            FuncVal::FreeStaticPrim(f) => Some(f.id.clone()),
            FuncVal::FreeStaticComp(_) => None,
            FuncVal::ConstCellPrim(f) => Some(f.id.clone()),
            FuncVal::ConstCellComp(_) => None,
            FuncVal::ConstStaticPrim(f) => Some(f.id.clone()),
            FuncVal::ConstStaticComp(_) => None,
            FuncVal::MutCellPrim(f) => Some(f.id.clone()),
            FuncVal::MutCellComp(_) => None,
            FuncVal::MutStaticPrim(f) => Some(f.id.clone()),
            FuncVal::MutStaticComp(_) => None,
        }
    }

    pub fn ctx(&self) -> Option<&Ctx> {
        match self {
            FuncVal::FreeCellPrim(_) => None,
            FuncVal::FreeCellComp(f) => Some(&f.ctx),
            FuncVal::FreeStaticPrim(_) => None,
            FuncVal::FreeStaticComp(f) => Some(&f.ctx),
            FuncVal::ConstCellPrim(_) => None,
            FuncVal::ConstCellComp(f) => Some(&f.ctx),
            FuncVal::ConstStaticPrim(_) => None,
            FuncVal::ConstStaticComp(f) => Some(&f.ctx),
            FuncVal::MutCellPrim(_) => None,
            FuncVal::MutCellComp(f) => Some(&f.ctx),
            FuncVal::MutStaticPrim(_) => None,
            FuncVal::MutStaticComp(f) => Some(&f.ctx),
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            FuncVal::FreeCellPrim(_) => true,
            FuncVal::FreeCellComp(_) => false,
            FuncVal::FreeStaticPrim(_) => true,
            FuncVal::FreeStaticComp(_) => false,
            FuncVal::ConstCellPrim(_) => true,
            FuncVal::ConstCellComp(_) => false,
            FuncVal::ConstStaticPrim(_) => true,
            FuncVal::ConstStaticComp(_) => false,
            FuncVal::MutCellPrim(_) => true,
            FuncVal::MutCellComp(_) => false,
            FuncVal::MutStaticPrim(_) => true,
            FuncVal::MutStaticComp(_) => false,
        }
    }

    pub fn is_cell(&self) -> bool {
        match self {
            FuncVal::FreeCellPrim(_) => true,
            FuncVal::FreeCellComp(_) => true,
            FuncVal::FreeStaticPrim(_) => false,
            FuncVal::FreeStaticComp(_) => false,
            FuncVal::ConstCellPrim(_) => true,
            FuncVal::ConstCellComp(_) => true,
            FuncVal::ConstStaticPrim(_) => false,
            FuncVal::ConstStaticComp(_) => false,
            FuncVal::MutCellPrim(_) => true,
            FuncVal::MutCellComp(_) => true,
            FuncVal::MutStaticPrim(_) => false,
            FuncVal::MutStaticComp(_) => false,
        }
    }

    pub fn ctx_access(&self) -> CtxAccess {
        match self {
            FuncVal::FreeCellPrim(_) => CtxAccess::Free,
            FuncVal::FreeCellComp(_) => CtxAccess::Free,
            FuncVal::FreeStaticPrim(_) => CtxAccess::Free,
            FuncVal::FreeStaticComp(_) => CtxAccess::Free,
            FuncVal::ConstCellPrim(_) => CtxAccess::Const,
            FuncVal::ConstCellComp(_) => CtxAccess::Const,
            FuncVal::ConstStaticPrim(_) => CtxAccess::Const,
            FuncVal::ConstStaticComp(_) => CtxAccess::Const,
            FuncVal::MutCellPrim(_) => CtxAccess::Mut,
            FuncVal::MutCellComp(_) => CtxAccess::Mut,
            FuncVal::MutStaticPrim(_) => CtxAccess::Mut,
            FuncVal::MutStaticComp(_) => CtxAccess::Mut,
        }
    }
}

impl From<FreeCellPrimFuncVal> for FuncVal {
    fn from(value: FreeCellPrimFuncVal) -> Self {
        Self::FreeCellPrim(value)
    }
}

impl From<FreeCellCompFuncVal> for FuncVal {
    fn from(value: FreeCellCompFuncVal) -> Self {
        Self::FreeCellComp(value)
    }
}

impl From<FreeStaticPrimFuncVal> for FuncVal {
    fn from(value: FreeStaticPrimFuncVal) -> Self {
        Self::FreeStaticPrim(value)
    }
}

impl From<FreeStaticCompFuncVal> for FuncVal {
    fn from(value: FreeStaticCompFuncVal) -> Self {
        Self::FreeStaticComp(value)
    }
}

impl From<ConstCellPrimFuncVal> for FuncVal {
    fn from(value: ConstCellPrimFuncVal) -> Self {
        Self::ConstCellPrim(value)
    }
}

impl From<ConstCellCompFuncVal> for FuncVal {
    fn from(value: ConstCellCompFuncVal) -> Self {
        Self::ConstCellComp(value)
    }
}

impl From<ConstStaticPrimFuncVal> for FuncVal {
    fn from(value: ConstStaticPrimFuncVal) -> Self {
        Self::ConstStaticPrim(value)
    }
}

impl From<ConstStaticCompFuncVal> for FuncVal {
    fn from(value: ConstStaticCompFuncVal) -> Self {
        Self::ConstStaticComp(value)
    }
}

impl From<MutCellPrimFuncVal> for FuncVal {
    fn from(value: MutCellPrimFuncVal) -> Self {
        Self::MutCellPrim(value)
    }
}

impl From<MutCellCompFuncVal> for FuncVal {
    fn from(value: MutCellCompFuncVal) -> Self {
        Self::MutCellComp(value)
    }
}

impl From<MutStaticPrimFuncVal> for FuncVal {
    fn from(value: MutStaticPrimFuncVal) -> Self {
        Self::MutStaticPrim(value)
    }
}

impl From<MutStaticCompFuncVal> for FuncVal {
    fn from(value: MutStaticCompFuncVal) -> Self {
        Self::MutStaticComp(value)
    }
}

impl Debug for FuncVal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match_func_val!(self, f => f.fmt(formatter))
    }
}
