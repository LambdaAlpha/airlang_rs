use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    AbstractMode,
    AskMode,
    CallMode,
    CompMode,
    FuncMode,
    Id,
    ListMode,
    MapMode,
    Mode,
    PairMode,
    PrimMode,
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
}

impl ModeFunc {
    pub fn new(mode: Mode) -> ModeFunc {
        let cacheable = mode.is_cacheable();
        Self { mode, cacheable }
    }

    pub fn self_mode(&self) -> &Mode {
        &self.mode
    }

    pub(crate) fn is_primitive(&self) -> bool {
        matches!(self.mode, Mode::Uni(_) | Mode::Prim(_))
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
