use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    AbstractMode,
    AskMode,
    CallMode,
    CompMode,
    CtxAccess,
    FuncMode,
    ListMode,
    MapMode,
    PairMode,
    PrimMode,
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
    cacheable: bool,
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
            abstract1: None,
            ask: Some(Mode::Uni(UniMode { code: CodeMode::Eval, symbol: SymbolMode::Ref })),
        }
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn call(&self) -> Val {
        Val::default()
    }
}

impl ModeFunc {
    pub fn new(mode: Option<Mode>) -> ModeFunc {
        let cacheable = mode.is_cacheable();
        let ctx_access = mode.ctx_access();
        Self { mode, cacheable, ctx_access }
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

trait IsCacheable {
    fn is_cacheable(&self) -> bool;
}

impl<T: IsCacheable> IsCacheable for Option<T> {
    fn is_cacheable(&self) -> bool {
        match self {
            None => true,
            Some(mode) => mode.is_cacheable(),
        }
    }
}

impl IsCacheable for Mode {
    fn is_cacheable(&self) -> bool {
        match self {
            Mode::Uni(mode) => mode.is_cacheable(),
            Mode::Prim(mode) => mode.is_cacheable(),
            Mode::Comp(mode) => mode.is_cacheable(),
            Mode::Func(mode) => mode.cacheable(),
        }
    }
}

impl IsCacheable for UniMode {
    fn is_cacheable(&self) -> bool {
        !matches!(self.code, CodeMode::Eval)
    }
}

impl IsCacheable for PairMode {
    fn is_cacheable(&self) -> bool {
        self.pair.first.is_cacheable() && self.pair.second.is_cacheable()
    }
}

impl IsCacheable for CallMode {
    fn is_cacheable(&self) -> bool {
        if matches!(self.code, CodeMode::Eval) {
            return false;
        }
        self.call.func.is_cacheable() && self.call.input.is_cacheable()
    }
}

impl IsCacheable for AbstractMode {
    fn is_cacheable(&self) -> bool {
        if matches!(self.code, CodeMode::Eval) {
            return false;
        }
        self.abstract1.func.is_cacheable() && self.abstract1.input.is_cacheable()
    }
}

impl IsCacheable for AskMode {
    fn is_cacheable(&self) -> bool {
        if matches!(self.code, CodeMode::Eval) {
            return false;
        }
        self.ask.func.is_cacheable() && self.ask.output.is_cacheable()
    }
}

impl IsCacheable for ListMode {
    fn is_cacheable(&self) -> bool {
        let head = self.head.iter().all(IsCacheable::is_cacheable);
        let tail = self.tail.is_cacheable();
        head && tail
    }
}

impl IsCacheable for MapMode {
    fn is_cacheable(&self) -> bool {
        let some = self.some.values().all(IsCacheable::is_cacheable);
        let else1 = self.else1.first.is_cacheable() && self.else1.second.is_cacheable();
        some && else1
    }
}

impl IsCacheable for CompMode {
    fn is_cacheable(&self) -> bool {
        self.pair.is_cacheable()
            && self.abstract1.is_cacheable()
            && self.call.is_cacheable()
            && self.ask.is_cacheable()
            && self.list.is_cacheable()
            && self.map.is_cacheable()
    }
}

impl IsCacheable for DataMode {
    fn is_cacheable(&self) -> bool {
        true
    }
}

impl IsCacheable for CodeMode {
    fn is_cacheable(&self) -> bool {
        *self != CodeMode::Eval
    }
}

impl IsCacheable for PrimMode {
    fn is_cacheable(&self) -> bool {
        self.pair.is_cacheable()
            && self.abstract1.is_cacheable()
            && self.call.is_cacheable()
            && self.ask.is_cacheable()
            && self.list.is_cacheable()
            && self.map.is_cacheable()
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
            & self.abstract1.ctx_access()
            & self.ask.ctx_access()
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
            & self.abstract1.ctx_access()
            & self.ask.ctx_access()
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

impl GetCtxAccess for AbstractMode {
    fn ctx_access(&self) -> CtxAccess {
        if matches!(self.code, CodeMode::Eval) {
            return CtxAccess::Mut;
        }
        self.abstract1.func.ctx_access() & self.abstract1.input.ctx_access()
    }
}

impl GetCtxAccess for AskMode {
    fn ctx_access(&self) -> CtxAccess {
        if matches!(self.code, CodeMode::Eval) {
            return CtxAccess::Mut;
        }
        self.ask.func.ctx_access() & self.ask.output.ctx_access()
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
