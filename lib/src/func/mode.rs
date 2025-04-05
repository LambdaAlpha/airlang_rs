use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    CallMode,
    CompMode,
    CtxAccess,
    FuncMode,
    ListMode,
    MapMode,
    OptimizeMode,
    PairMode,
    PrimMode,
    SolveMode,
    SymbolMode,
    UniMode,
    Val,
    ctx::ref1::CtxMeta,
    func::FuncTrait,
    mode::{
        Mode,
        prim::{
            CodeMode,
            DataMode,
        },
    },
    transformer::Transformer,
};

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
        &FuncMode {
            call: None,
            optimize: None,
            solve: Some(Mode::Uni(UniMode { code: CodeMode::Eval, symbol: SymbolMode::Ref })),
        }
    }

    fn call(&self) -> Val {
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
            & self.optimize.ctx_access()
            & self.solve.ctx_access()
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
            & self.optimize.ctx_access()
            & self.solve.ctx_access()
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
        self.pair.first.ctx_access() & self.pair.second.ctx_access()
    }
}

impl GetCtxAccess for CallMode {
    fn ctx_access(&self) -> CtxAccess {
        if matches!(self.code, CodeMode::Eval) {
            return CtxAccess::Mut;
        }
        self.call.func.ctx_access() & self.call.input.ctx_access()
    }
}

impl GetCtxAccess for OptimizeMode {
    fn ctx_access(&self) -> CtxAccess {
        self.optimize.func.ctx_access()
    }
}

impl GetCtxAccess for SolveMode {
    fn ctx_access(&self) -> CtxAccess {
        self.solve.func.ctx_access()
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
