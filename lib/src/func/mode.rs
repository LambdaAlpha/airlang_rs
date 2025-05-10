use std::fmt::Debug;
use std::fmt::Formatter;

use crate::CallMode;
use crate::CompMode;
use crate::CtxAccess;
use crate::FuncMode;
use crate::ListMode;
use crate::MapMode;
use crate::PairMode;
use crate::PrimMode;
use crate::SymbolMode;
use crate::UniMode;
use crate::Val;
use crate::ctx::ref1::CtxMeta;
use crate::func::FuncTrait;
use crate::mode::Mode;
use crate::mode::prim::CodeMode;
use crate::mode::prim::DataMode;
use crate::transformer::Transformer;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ModeFunc {
    mode: Option<Mode>,
    ctx_access: CtxAccess,
}

impl Transformer<Val, Val> for ModeFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.mode.transform(ctx, input)
    }
}

impl FuncTrait for ModeFunc {
    fn mode(&self) -> &FuncMode {
        &FuncMode { forward: None, reverse: None }
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
                Mode::Uni(_) => true,
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
            Mode::Uni(mode) => mode.ctx_access(),
            Mode::Prim(mode) => mode.ctx_access(),
            Mode::Comp(mode) => mode.ctx_access(),
            Mode::Func(mode) => mode.ctx_access(),
        }
    }
}

impl GetCtxAccess for UniMode {
    fn ctx_access(&self) -> CtxAccess {
        CtxAccess::Mut
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
        if matches!(self.code, CodeMode::Eval) {
            return CtxAccess::Mut;
        }
        self.func.ctx_access() & self.input.ctx_access()
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
        let else1 = self.else1.first.ctx_access() & self.else1.second.ctx_access();
        some & else1
    }
}
