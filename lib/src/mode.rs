use crate::{
    Val,
    ctx::{
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    mode::{
        composite::CompositeMode,
        primitive::PrimitiveMode,
        recursive::SelfMode,
    },
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Primitive(PrimitiveMode),
    Recursive(CompositeMode<SelfMode>),
    Composite(Box<CompositeMode<Mode>>),
}

impl Transformer<Val, Val> for Mode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Mode::Primitive(mode) => mode.transform(ctx, input),
            Mode::Recursive(mode) => mode.transform(ctx, input),
            Mode::Composite(mode) => mode.transform(ctx, input),
        }
    }
}

impl Mode {
    pub fn apply(&self, ctx: MutFnCtx, val: Val) -> Val {
        self.transform(ctx, val)
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Primitive(PrimitiveMode::default())
    }
}

impl From<PrimitiveMode> for Mode {
    fn from(mode: PrimitiveMode) -> Self {
        Mode::Primitive(mode)
    }
}

pub(crate) mod id;

pub(crate) mod form;

pub(crate) mod eval;

pub(crate) mod primitive;

pub(crate) mod recursive;

pub(crate) mod composite;

pub(crate) mod symbol;

pub(crate) mod pair;

pub(crate) mod call;

pub(crate) mod abstract1;

pub(crate) mod ask;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod repr;
