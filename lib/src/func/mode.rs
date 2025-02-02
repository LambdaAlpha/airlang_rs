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
    Id,
    ListMode,
    MapMode,
    Mode,
    PairMode,
    PrimMode,
    SymbolMode,
    UniMode,
    Val,
    ctx::ref1::CtxMeta,
    func::FuncTrait,
    mode::{
        eval::{
            EVAL,
            EvalMode,
        },
        form::FormMode,
    },
    transformer::Transformer,
};

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct ModeFunc {
    mode: Mode,
    cacheable: bool,
    ctx_access: CtxAccess,
}

impl Transformer<Val, Val> for ModeFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.mode.transform(ctx, input)
    }
}

impl FuncTrait for ModeFunc {
    fn mode(&self) -> &FuncMode {
        &FuncMode {
            call: Mode::Uni(UniMode::Id(Id)),
            abstract1: Mode::Uni(UniMode::Id(Id)),
            ask: Mode::Uni(UniMode::Eval(EVAL)),
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
    pub fn new(mode: Mode) -> ModeFunc {
        let cacheable = mode.is_cacheable();
        let ctx_access = mode.ctx_access();
        Self {
            mode,
            cacheable,
            ctx_access,
        }
    }

    pub fn self_mode(&self) -> &Mode {
        &self.mode
    }

    pub(crate) fn is_primitive(&self) -> bool {
        matches!(self.mode, Mode::Uni(_) | Mode::Prim(_))
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
        !matches!(self, UniMode::Eval(_))
    }
}

impl IsCacheable for PairMode {
    fn is_cacheable(&self) -> bool {
        match self {
            PairMode::Id(_) => true,
            PairMode::Form(mode) => mode.first.is_cacheable() && mode.second.is_cacheable(),
        }
    }
}

impl IsCacheable for AbstractMode {
    fn is_cacheable(&self) -> bool {
        match self {
            AbstractMode::Id(_) => true,
            AbstractMode::Form(mode) => mode.func.is_cacheable() && mode.input.is_cacheable(),
            AbstractMode::Eval(mode) => mode.func.is_cacheable() && mode.input.is_cacheable(),
        }
    }
}

impl IsCacheable for CallMode {
    fn is_cacheable(&self) -> bool {
        match self {
            CallMode::Id(_) => true,
            CallMode::Form(mode) => mode.func.is_cacheable() && mode.input.is_cacheable(),
            CallMode::Eval(_) => false,
        }
    }
}

impl IsCacheable for AskMode {
    fn is_cacheable(&self) -> bool {
        match self {
            AskMode::Id(_) => true,
            AskMode::Form(mode) => mode.func.is_cacheable() && mode.output.is_cacheable(),
            AskMode::Eval(_) => false,
        }
    }
}

impl IsCacheable for ListMode {
    fn is_cacheable(&self) -> bool {
        match self {
            ListMode::Id(_) => true,
            ListMode::Form { head, tail } => {
                let head = head.iter().all(Mode::is_cacheable);
                let tail = tail.is_cacheable();
                head && tail
            }
        }
    }
}

impl IsCacheable for MapMode {
    fn is_cacheable(&self) -> bool {
        match self {
            MapMode::Id(_) => true,
            MapMode::Form { some, else1 } => {
                let some = some.values().all(Mode::is_cacheable);
                let else1 = else1.first.is_cacheable() && else1.second.is_cacheable();
                some && else1
            }
        }
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

impl IsCacheable for FormMode {
    fn is_cacheable(&self) -> bool {
        true
    }
}

impl IsCacheable for EvalMode {
    fn is_cacheable(&self) -> bool {
        *self != EvalMode::Eval
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
        match self {
            UniMode::Id(_) => CtxAccess::Free,
            UniMode::Form(_) => CtxAccess::Mut,
            UniMode::Eval(_) => CtxAccess::Mut,
        }
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

impl GetCtxAccess for FormMode {
    fn ctx_access(&self) -> CtxAccess {
        CtxAccess::Free
    }
}

impl GetCtxAccess for EvalMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            EvalMode::Id => CtxAccess::Free,
            EvalMode::Form => CtxAccess::Free,
            EvalMode::Eval => CtxAccess::Mut,
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
        match self {
            SymbolMode::Id(_) => CtxAccess::Free,
            SymbolMode::Form(_) => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for PairMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            PairMode::Id(_) => CtxAccess::Free,
            PairMode::Form(mode) => mode.first.ctx_access() & mode.second.ctx_access(),
        }
    }
}

impl GetCtxAccess for CallMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            CallMode::Id(_) => CtxAccess::Free,
            CallMode::Form(mode) => mode.func.ctx_access() & mode.input.ctx_access(),
            CallMode::Eval(_) => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for AbstractMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            AbstractMode::Id(_) => CtxAccess::Free,
            AbstractMode::Form(mode) => mode.func.ctx_access() & mode.input.ctx_access(),
            AbstractMode::Eval(_) => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for AskMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            AskMode::Id(_) => CtxAccess::Free,
            AskMode::Form(mode) => mode.func.ctx_access() & mode.output.ctx_access(),
            AskMode::Eval(_) => CtxAccess::Mut,
        }
    }
}

impl GetCtxAccess for ListMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            ListMode::Id(_) => CtxAccess::Free,
            ListMode::Form { head, tail } => {
                let head = head
                    .iter()
                    .fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
                let tail = tail.ctx_access();
                head & tail
            }
        }
    }
}

impl GetCtxAccess for MapMode {
    fn ctx_access(&self) -> CtxAccess {
        match self {
            MapMode::Id(_) => CtxAccess::Free,
            MapMode::Form { some, else1 } => {
                let some = some
                    .values()
                    .fold(CtxAccess::Free, |access, mode| access & mode.ctx_access());
                let else1 = else1.first.ctx_access() & else1.second.ctx_access();
                some & else1
            }
        }
    }
}
