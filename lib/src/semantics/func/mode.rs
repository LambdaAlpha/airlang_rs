use std::fmt::Debug;
use std::fmt::Formatter;

use crate::semantics::ctx::CtxAccess;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FuncMode;
use crate::semantics::func::FuncTrait;
use crate::semantics::func::MutStaticFn;
use crate::semantics::mode::CallMode;
use crate::semantics::mode::CodeMode;
use crate::semantics::mode::CompMode;
use crate::semantics::mode::DataMode;
use crate::semantics::mode::ListMode;
use crate::semantics::mode::MapMode;
use crate::semantics::mode::Mode;
use crate::semantics::mode::PairMode;
use crate::semantics::mode::PrimMode;
use crate::semantics::mode::SymbolMode;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ModeFunc {
    mode: Option<Mode>,
    ctx_access: CtxAccess,
}

impl FreeStaticFn<Val, Val> for ModeFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.mode.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ModeFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.mode.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Val, Val> for ModeFunc {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        self.mode.mut_static_call(ctx, input)
    }
}

impl FuncTrait for ModeFunc {
    fn mode(&self) -> &FuncMode {
        &FuncMode { forward: None, reverse: None }
    }

    fn ctx_explicit(&self) -> bool {
        false
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl ModeFunc {
    pub fn new(mode: Option<Mode>) -> ModeFunc {
        let ctx_access = mode.ctx_access();
        Self { mode, ctx_access }
    }

    pub fn inner(&self) -> &Option<Mode> {
        &self.mode
    }

    pub(crate) fn is_primitive(&self) -> bool {
        match &self.mode {
            None => true,
            Some(mode) => match mode {
                Mode::Prim(_) => true,
                Mode::Comp(_) => false,
                Mode::Func(_) => false,
            },
        }
    }

    pub(crate) fn ctx_access(&self) -> CtxAccess {
        self.ctx_access
    }
}

impl Debug for ModeFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ModeFunc");
        s.field("mode", &self.mode);
        s.finish()
    }
}

trait GetCtxAccess {
    fn ctx_access(&self) -> CtxAccess;
}

impl<T: GetCtxAccess> GetCtxAccess for Option<T> {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            None => CtxAccess::Free,
            Some(mode) => mode.ctx_access(),
        }
    }
}

impl GetCtxAccess for Mode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            Mode::Prim(mode) => mode.ctx_access(),
            Mode::Comp(mode) => mode.ctx_access(),
            Mode::Func(mode) => mode.ctx_access(),
        }
    }
}

impl GetCtxAccess for PrimMode {
    fn ctx_access(&self) -> CtxAccess {
        self.symbol.ctx_access()
            & self.pair.ctx_access()
            & self.call.ctx_access()
            & self.list.ctx_access()
            & self.map.ctx_access()
    }
}

impl GetCtxAccess for DataMode {
    fn ctx_access(&self) -> CtxAccess {
        CtxAccess::Free
    }
}

impl GetCtxAccess for CodeMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            CodeMode::Form => CtxAccess::Free,
            CodeMode::Eval => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for CompMode {
    fn ctx_access(&self) -> CtxAccess {
        self.symbol.ctx_access()
            & self.pair.ctx_access()
            & self.call.ctx_access()
            & self.list.ctx_access()
            & self.map.ctx_access()
    }
}

impl GetCtxAccess for SymbolMode {
    fn ctx_access(&self) -> CtxAccess {
        CtxAccess::Mut
    }
}

impl GetCtxAccess for PairMode {
    fn ctx_access(&self) -> CtxAccess {
        self.first.ctx_access() & self.second.ctx_access()
    }
}

impl GetCtxAccess for CallMode {
    fn ctx_access(&self) -> CtxAccess {
        match &self.some {
            None => CtxAccess::Mut,
            Some(some) => {
                let some =
                    some.values().fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
                let else_ = self.func.ctx_access() & self.input.ctx_access();
                some & else_
            }
        }
    }
}

impl GetCtxAccess for ListMode {
    fn ctx_access(&self) -> CtxAccess {
        let head =
            self.head.iter().fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
        let tail = self.tail.ctx_access();
        head & tail
    }
}

impl GetCtxAccess for MapMode {
    fn ctx_access(&self) -> CtxAccess {
        let some =
            self.some.values().fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
        let else_ = self.else_.first.ctx_access() & self.else_.second.ctx_access();
        some & else_
    }
}
