pub use self::const_cell_comp::ConstCellCompFuncVal;
pub use self::const_cell_prim::ConstCellPrimFuncVal;
pub use self::const_static_comp::ConstStaticCompFuncVal;
pub use self::const_static_prim::ConstStaticPrimFuncVal;
pub use self::free_cell_comp::FreeCellCompFuncVal;
pub use self::free_cell_prim::FreeCellPrimFuncVal;
pub use self::free_static_comp::FreeStaticCompFuncVal;
pub use self::free_static_prim::FreeStaticPrimFuncVal;
pub use self::mode::ModeFuncVal;
pub use self::mut_cell_comp::MutCellCompFuncVal;
pub use self::mut_cell_prim::MutCellPrimFuncVal;
pub use self::mut_static_comp::MutStaticCompFuncVal;
pub use self::mut_static_prim::MutStaticPrimFuncVal;

_____!();

use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;

use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstCellFn;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeCellFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FuncMode;
use crate::semantics::func::FuncTrait;
use crate::semantics::func::MutCellFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::mode::ModeFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

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

impl ConstStaticFn<Val, Val, Val> for FuncVal {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
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

impl ConstCellFn<Val, Val, Val> for FuncVal {
    fn const_cell_call(&mut self, ctx: ConstRef<Val>, input: Val) -> Val {
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

impl MutStaticFn<Val, Val, Val> for FuncVal {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
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

impl MutCellFn<Val, Val, Val> for FuncVal {
    fn mut_cell_call(&mut self, ctx: &mut Val, input: Val) -> Val {
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

    fn ctx_explicit(&self) -> bool {
        match self {
            FuncVal::Mode(f) => f.ctx_explicit(),
            FuncVal::FreeCellPrim(f) => f.ctx_explicit(),
            FuncVal::FreeCellComp(f) => f.ctx_explicit(),
            FuncVal::FreeStaticPrim(f) => f.ctx_explicit(),
            FuncVal::FreeStaticComp(f) => f.ctx_explicit(),
            FuncVal::ConstCellPrim(f) => f.ctx_explicit(),
            FuncVal::ConstCellComp(f) => f.ctx_explicit(),
            FuncVal::ConstStaticPrim(f) => f.ctx_explicit(),
            FuncVal::ConstStaticComp(f) => f.ctx_explicit(),
            FuncVal::MutCellPrim(f) => f.ctx_explicit(),
            FuncVal::MutCellComp(f) => f.ctx_explicit(),
            FuncVal::MutStaticPrim(f) => f.ctx_explicit(),
            FuncVal::MutStaticComp(f) => f.ctx_explicit(),
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
    pub fn id(&self) -> Option<Symbol> {
        match self {
            FuncVal::Mode(_) => None,
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
            FuncVal::Mode(_) => None,
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

    pub fn is_cell(&self) -> bool {
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

    pub fn ctx_access(&self) -> CtxAccess {
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

impl From<ModeFuncVal> for FuncVal {
    fn from(value: ModeFuncVal) -> Self {
        Self::Mode(value)
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
    ($type_:ty) => {
        impl $crate::semantics::func::FuncTrait for $type_ {
            fn mode(&self) -> &$crate::semantics::func::FuncMode {
                self.0.mode()
            }

            fn ctx_explicit(&self) -> bool {
                self.0.ctx_explicit()
            }

            fn code(&self) -> $crate::semantics::val::Val {
                self.0.code()
            }
        }
    };
}

#[expect(unused)]
pub(crate) use impl_func_trait;

mod mode;

mod free_cell_prim;

mod free_cell_comp;

mod free_static_prim;

mod free_static_comp;

mod const_cell_prim;

mod const_cell_comp;

mod const_static_prim;

mod const_static_comp;

mod mut_cell_prim;

mod mut_cell_comp;

mod mut_static_prim;

mod mut_static_comp;
