use crate::{
    AbstractVal,
    AskVal,
    CallVal,
    FormMode,
    PairVal,
    UniMode,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::{
        id::Id,
        symbol::PrefixMode,
    },
    symbol::Symbol,
    transformer::{
        ByVal,
        Transformer,
    },
    val::{
        Val,
        list::ListVal,
        map::MapVal,
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Eval {
    prefix: PrefixMode,
}

// default instance
pub(crate) const EVAL: Eval = Eval::new(PrefixMode::Ref);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EvalMode {
    Id,
    Form,
    #[default]
    Eval,
}

impl Transformer<Val, Val> for Eval {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Eval {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.prefix.transform(ctx, symbol)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_pair(self, self, ctx, pair)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        EvalCore::transform_call(self, self, ctx, call)
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        EvalCore::transform_abstract(self, self, ctx, abstract1)
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        EvalCore::transform_ask(self, self, ctx, ask)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_list(self, ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_map(self, self, ctx, map)
    }
}

impl Eval {
    pub const fn new(prefix: PrefixMode) -> Self {
        Eval { prefix }
    }

    pub fn prefix_mode(&self) -> PrefixMode {
        self.prefix
    }
}

impl From<UniMode> for EvalMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(_) => EvalMode::Id,
            UniMode::Form(_) => EvalMode::Form,
            UniMode::Eval(_) => EvalMode::Eval,
        }
    }
}

impl From<FormMode> for EvalMode {
    fn from(mode: FormMode) -> Self {
        match mode {
            FormMode::Id => EvalMode::Id,
            FormMode::Form => EvalMode::Form,
        }
    }
}
