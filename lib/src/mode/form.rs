use crate::{
    ctx::{
        ref1::CtxMeta,
        DefaultCtx,
    },
    mode::{
        id::Id,
        SYMBOL_MOVE_PREFIX,
        SYMBOL_READ_PREFIX,
    },
    symbol::Symbol,
    transformer::{
        input::ByVal,
        DefaultByVal,
        Transformer,
    },
    val::{
        list::ListVal,
        map::MapVal,
        Val,
    },
    AskVal,
    CallVal,
    CommentVal,
    PairVal,
};

#[derive(Copy, Clone)]
pub(crate) struct Form;

impl Transformer<Val, Val> for Form {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Form {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match s.chars().next() {
            Some(Symbol::ID_PREFIX) => {
                let s = Symbol::from_str(&s[1..]);
                Id.transform_symbol(ctx, s)
            }
            Some(SYMBOL_READ_PREFIX) => {
                let s = Symbol::from_str(&s[1..]);
                DefaultCtx.get_or_default(ctx, s)
            }
            Some(SYMBOL_MOVE_PREFIX) => {
                let s = Symbol::from_str(&s[1..]);
                ctx.remove(s).unwrap_or_default()
            }
            _ => Id.transform_symbol(ctx, s),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_pair(self, ctx, pair)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_list(self, ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_map(self, ctx, map)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_call(self, ctx, call)
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_ask(self, ctx, ask)
    }

    fn transform_comment<'a, Ctx>(&self, ctx: Ctx, comment: CommentVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_comment(self, ctx, comment)
    }
}
