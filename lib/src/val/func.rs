use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;

use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::CtxAccess;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::Composite;
use crate::func::prim::Primitive;
use crate::mode::ModeFn;
use crate::val::func::const_cell_comp::ConstCellCompFuncVal;
use crate::val::func::const_cell_prim::ConstCellPrimFuncVal;
use crate::val::func::const_static_comp::ConstStaticCompFuncVal;
use crate::val::func::const_static_prim::ConstStaticPrimFuncVal;
use crate::val::func::free_cell_comp::FreeCellCompFuncVal;
use crate::val::func::free_cell_prim::FreeCellPrimFuncVal;
use crate::val::func::free_static_comp::FreeStaticCompFuncVal;
use crate::val::func::free_static_prim::FreeStaticPrimFuncVal;
use crate::val::func::mode::ModeFuncVal;
use crate::val::func::mut_cell_comp::MutCellCompFuncVal;
use crate::val::func::mut_cell_prim::MutCellPrimFuncVal;
use crate::val::func::mut_static_comp::MutStaticCompFuncVal;
use crate::val::func::mut_static_prim::MutStaticPrimFuncVal;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FuncVal {
    Mode(ModeFuncVal),
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

impl ModeFn for FuncVal {}

impl FreeStaticFn<Val, Val> for FuncVal {
    fn free_static_call(&self, input: Val) -> Val {
        match self {
            FuncVal::Mode(f) => f.free_static_call(input),
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
            FuncVal::Mode(f) => f.free_static_call(input),
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

impl ConstStaticFn<Ctx, Val, Val> for FuncVal {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        match self {
            FuncVal::Mode(f) => f.const_static_call(ctx, input),
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

impl ConstCellFn<Ctx, Val, Val> for FuncVal {
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        match self {
            FuncVal::Mode(f) => f.const_static_call(ctx, input),
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

impl MutStaticFn<Ctx, Val, Val> for FuncVal {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncVal::Mode(f) => f.mut_static_call(ctx, input),
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

impl MutCellFn<Ctx, Val, Val> for FuncVal {
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            FuncVal::Mode(f) => f.mut_static_call(ctx, input),
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

impl FuncTrait for FuncVal {
    fn mode(&self) -> &FuncMode {
        match self {
            FuncVal::Mode(f) => f.mode(),
            FuncVal::FreeCellPrim(f) => f.mode(),
            FuncVal::FreeCellComp(f) => f.mode(),
            FuncVal::FreeStaticPrim(f) => f.mode(),
            FuncVal::FreeStaticComp(f) => f.mode(),
            FuncVal::ConstCellPrim(f) => f.mode(),
            FuncVal::ConstCellComp(f) => f.mode(),
            FuncVal::ConstStaticPrim(f) => f.mode(),
            FuncVal::ConstStaticComp(f) => f.mode(),
            FuncVal::MutCellPrim(f) => f.mode(),
            FuncVal::MutCellComp(f) => f.mode(),
            FuncVal::MutStaticPrim(f) => f.mode(),
            FuncVal::MutStaticComp(f) => f.mode(),
        }
    }

    fn code(&self) -> Val {
        match self {
            FuncVal::Mode(f) => f.code(),
            FuncVal::FreeCellPrim(f) => f.code(),
            FuncVal::FreeCellComp(f) => f.code(),
            FuncVal::FreeStaticPrim(f) => f.code(),
            FuncVal::FreeStaticComp(f) => f.code(),
            FuncVal::ConstCellPrim(f) => f.code(),
            FuncVal::ConstCellComp(f) => f.code(),
            FuncVal::ConstStaticPrim(f) => f.code(),
            FuncVal::ConstStaticComp(f) => f.code(),
            FuncVal::MutCellPrim(f) => f.code(),
            FuncVal::MutCellComp(f) => f.code(),
            FuncVal::MutStaticPrim(f) => f.code(),
            FuncVal::MutStaticComp(f) => f.code(),
        }
    }
}

impl FuncVal {
    pub(crate) fn primitive(&self) -> Option<&Primitive> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::FreeCellPrim(f) => Some(&f.prim),
            FuncVal::FreeCellComp(_) => None,
            FuncVal::FreeStaticPrim(f) => Some(&f.prim),
            FuncVal::FreeStaticComp(_) => None,
            FuncVal::ConstCellPrim(f) => Some(&f.prim),
            FuncVal::ConstCellComp(_) => None,
            FuncVal::ConstStaticPrim(f) => Some(&f.prim),
            FuncVal::ConstStaticComp(_) => None,
            FuncVal::MutCellPrim(f) => Some(&f.prim),
            FuncVal::MutCellComp(_) => None,
            FuncVal::MutStaticPrim(f) => Some(&f.prim),
            FuncVal::MutStaticComp(_) => None,
        }
    }

    pub(crate) fn composite(&self) -> Option<&Composite> {
        match self {
            FuncVal::Mode(_) => None,
            FuncVal::FreeCellPrim(_) => None,
            FuncVal::FreeCellComp(f) => Some(&f.comp),
            FuncVal::FreeStaticPrim(_) => None,
            FuncVal::FreeStaticComp(f) => Some(&f.comp),
            FuncVal::ConstCellPrim(_) => None,
            FuncVal::ConstCellComp(f) => Some(&f.comp),
            FuncVal::ConstStaticPrim(_) => None,
            FuncVal::ConstStaticComp(f) => Some(&f.comp),
            FuncVal::MutCellPrim(_) => None,
            FuncVal::MutCellComp(f) => Some(&f.comp),
            FuncVal::MutStaticPrim(_) => None,
            FuncVal::MutStaticComp(f) => Some(&f.comp),
        }
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.is_primitive(),
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

    pub(crate) fn is_cell(&self) -> bool {
        match self {
            FuncVal::Mode(_) => false,
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

    pub(crate) fn ctx_access(&self) -> CtxAccess {
        match self {
            FuncVal::Mode(mode) => mode.ctx_access(),
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

impl Debug for FuncVal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncVal::Mode(f) => f.fmt(formatter),
            FuncVal::FreeCellPrim(f) => f.fmt(formatter),
            FuncVal::FreeCellComp(f) => f.fmt(formatter),
            FuncVal::FreeStaticPrim(f) => f.fmt(formatter),
            FuncVal::FreeStaticComp(f) => f.fmt(formatter),
            FuncVal::ConstCellPrim(f) => f.fmt(formatter),
            FuncVal::ConstCellComp(f) => f.fmt(formatter),
            FuncVal::ConstStaticPrim(f) => f.fmt(formatter),
            FuncVal::ConstStaticComp(f) => f.fmt(formatter),
            FuncVal::MutCellPrim(f) => f.fmt(formatter),
            FuncVal::MutCellComp(f) => f.fmt(formatter),
            FuncVal::MutStaticPrim(f) => f.fmt(formatter),
            FuncVal::MutStaticComp(f) => f.fmt(formatter),
        }
    }
}

macro_rules! impl_func_trait {
    ($type1:ty) => {
        impl $crate::func::FuncTrait for $type1 {
            fn mode(&self) -> &$crate::func::func_mode::FuncMode {
                self.0.mode()
            }

            fn code(&self) -> $crate::val::Val {
                self.0.code()
            }
        }
    };
}

#[expect(unused)]
pub(crate) use impl_func_trait;

pub(crate) mod mode;

pub(crate) mod free_cell_prim;

pub(crate) mod free_cell_comp;

pub(crate) mod free_static_prim;

pub(crate) mod free_static_comp;

pub(crate) mod const_cell_prim;

pub(crate) mod const_cell_comp;

pub(crate) mod const_static_prim;

pub(crate) mod const_static_comp;

pub(crate) mod mut_cell_prim;

pub(crate) mod mut_cell_comp;

pub(crate) mod mut_static_prim;

pub(crate) mod mut_static_comp;
