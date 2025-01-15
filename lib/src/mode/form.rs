use crate::{
    AbstractVal,
    AskVal,
    CallVal,
    EvalMode,
    PairVal,
    UniMode,
    core::FormCore,
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
pub struct Form {
    prefix: PrefixMode,
}

// default instance
#[allow(unused)]
pub(crate) const FORM: Form = Form::new(PrefixMode::Ref);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FormMode {
    Id,
    #[default]
    Form,
}

impl Transformer<Val, Val> for Form {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Form {
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
        FormCore::transform_call(self, self, ctx, call)
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_abstract(self, self, ctx, abstract1)
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_ask(self, self, ctx, ask)
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

impl Form {
    pub const fn new(prefix: PrefixMode) -> Self {
        Form { prefix }
    }

    pub fn prefix_mode(&self) -> PrefixMode {
        self.prefix
    }
}

impl From<UniMode> for FormMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(_) => FormMode::Id,
            UniMode::Form(_) => FormMode::Form,
            UniMode::Eval(_) => FormMode::Form,
        }
    }
}

impl From<EvalMode> for FormMode {
    fn from(mode: EvalMode) -> Self {
        match mode {
            EvalMode::Id => FormMode::Id,
            EvalMode::Form => FormMode::Form,
            EvalMode::Eval => FormMode::Form,
        }
    }
}
