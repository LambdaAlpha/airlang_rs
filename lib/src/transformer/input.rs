use crate::{
    AskVal,
    CallVal,
    CommentVal,
    PairVal,
    Val,
    ctx::ref1::CtxMeta,
    symbol::Symbol,
    transformer::Transformer,
    val::{
        list::ListVal,
        map::MapVal,
    },
};

pub(crate) trait ByVal<Output>: Transformer<Val, Output> {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_comment<'a, Ctx>(&self, ctx: Ctx, comment: CommentVal) -> Output
    where
        Ctx: CtxMeta<'a>;
}
