use crate::{
    AbstractVal,
    AskVal,
    CallVal,
    Form,
    PairVal,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::id::Id,
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
pub enum Eval {
    Literal,
    #[default]
    Ref,
    Move,
}

// default instance
pub(crate) const EVAL: Eval = Eval::Ref;

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
        Form::from(*self).transform(ctx, symbol)
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

impl From<Form> for Eval {
    fn from(form: Form) -> Self {
        match form {
            Form::Literal => Eval::Literal,
            Form::Ref => Eval::Ref,
            Form::Move => Eval::Move,
        }
    }
}

impl From<Eval> for Form {
    fn from(eval: Eval) -> Self {
        match eval {
            Eval::Literal => Form::Literal,
            Eval::Ref => Form::Ref,
            Eval::Move => Form::Move,
        }
    }
}
