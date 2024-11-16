use crate::{
    AskMode,
    CallMode,
    CommentMode,
    CompositeMode,
    ListMode,
    MapMode,
    Mode,
    PairMode,
    PrimitiveMode,
    SelfMode,
    Val,
    ctx::ref1::CtxMeta,
    transformer::Transformer,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

impl ModeFunc {
    pub fn new(mode: Mode) -> ModeFunc {
        let cacheable = mode.is_cacheable();
        Self { mode, cacheable }
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn cacheable(&self) -> bool {
        self.cacheable
    }

    pub(crate) fn is_primitive(&self) -> bool {
        matches!(self.mode, Mode::Primitive(_))
    }
}

trait IsCacheable {
    fn is_cacheable(&self) -> bool;
}

impl IsCacheable for Mode {
    fn is_cacheable(&self) -> bool {
        match self {
            Mode::Primitive(mode) => mode.is_cacheable(),
            Mode::Recursive(mode) => mode.is_cacheable(),
            Mode::Composite(mode) => mode.is_cacheable(),
        }
    }
}

impl IsCacheable for PrimitiveMode {
    fn is_cacheable(&self) -> bool {
        !matches!(self, PrimitiveMode::Eval)
    }
}

impl IsCacheable for SelfMode {
    fn is_cacheable(&self) -> bool {
        match self {
            SelfMode::Self1 => true,
            SelfMode::Primitive(mode) => mode.is_cacheable(),
        }
    }
}

impl<M: IsCacheable> IsCacheable for PairMode<M> {
    fn is_cacheable(&self) -> bool {
        match self {
            PairMode::Id => true,
            PairMode::Form(mode) => mode.first.is_cacheable() && mode.second.is_cacheable(),
        }
    }
}

impl<M: IsCacheable> IsCacheable for CommentMode<M> {
    fn is_cacheable(&self) -> bool {
        match self {
            CommentMode::Id => true,
            CommentMode::Form(mode) => mode.meta.is_cacheable() && mode.value.is_cacheable(),
            CommentMode::Eval(mode) => mode.meta.is_cacheable() && mode.value.is_cacheable(),
        }
    }
}

impl<M: IsCacheable> IsCacheable for CallMode<M> {
    fn is_cacheable(&self) -> bool {
        match self {
            CallMode::Id => true,
            CallMode::Form(mode) => mode.func.is_cacheable() && mode.input.is_cacheable(),
            CallMode::Eval(_) => false,
        }
    }
}

impl<M: IsCacheable> IsCacheable for AskMode<M> {
    fn is_cacheable(&self) -> bool {
        match self {
            AskMode::Id => true,
            AskMode::Form(mode) => mode.func.is_cacheable() && mode.output.is_cacheable(),
            AskMode::Eval(_) => false,
        }
    }
}

impl<M: IsCacheable> IsCacheable for ListMode<M> {
    fn is_cacheable(&self) -> bool {
        match self {
            ListMode::Id => true,
            ListMode::Form { head, tail } => {
                let head = head.iter().all(M::is_cacheable);
                let tail = tail.is_cacheable();
                head && tail
            }
        }
    }
}

impl<M: IsCacheable> IsCacheable for MapMode<M> {
    fn is_cacheable(&self) -> bool {
        match self {
            MapMode::Id => true,
            MapMode::Form { some, else1 } => {
                let some = some.values().all(M::is_cacheable);
                let else1 = else1.first.is_cacheable() && else1.second.is_cacheable();
                some && else1
            }
        }
    }
}

impl<M: IsCacheable> IsCacheable for CompositeMode<M> {
    fn is_cacheable(&self) -> bool {
        self.pair.is_cacheable()
            && self.comment.is_cacheable()
            && self.call.is_cacheable()
            && self.ask.is_cacheable()
            && self.list.is_cacheable()
            && self.map.is_cacheable()
    }
}
