use crate::{
    FuncVal,
    Val,
    ctx::{
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    mode::{
        comp::CompMode,
        prim::PrimMode,
        united::UniMode,
    },
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Uni(UniMode),
    Prim(PrimMode),
    Comp(Box<CompMode>),
    Func(FuncVal),
}

impl Transformer<Val, Val> for Mode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        match self {
            Mode::Uni(mode) => mode.transform(ctx, input),
            Mode::Prim(mode) => mode.transform(ctx, input),
            Mode::Comp(mode) => mode.transform(ctx, input),
            Mode::Func(mode) => mode.transform(ctx, input),
        }
    }
}

impl Mode {
    pub fn apply(&self, ctx: MutFnCtx, val: Val) -> Val {
        self.transform(ctx, val)
    }
}

impl From<UniMode> for Mode {
    fn from(mode: UniMode) -> Self {
        Mode::Uni(mode)
    }
}

pub(crate) mod id;

pub(crate) mod form;

pub(crate) mod eval;

pub(crate) mod united;

pub(crate) mod prim;

pub(crate) mod comp;

pub(crate) mod symbol;

pub(crate) mod pair;

pub(crate) mod call;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod repr;
