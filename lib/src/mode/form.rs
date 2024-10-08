use crate::{
    AskVal,
    CallVal,
    CommentVal,
    PairVal,
    ctx::{
        default::DefaultCtx,
        map::CtxMapRef,
        ref1::CtxMeta,
    },
    mode::{
        SYMBOL_MOVE_PREFIX,
        SYMBOL_READ_PREFIX,
        id::Id,
    },
    symbol::Symbol,
    transformer::{
        DefaultByVal,
        Transformer,
        input::ByVal,
    },
    val::{
        Val,
        list::ListVal,
        map::MapVal,
    },
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
                let Ok(variables) = ctx.get_variables_mut() else {
                    return Val::default();
                };
                let s = Symbol::from_str(&s[1..]);
                variables.remove(s).unwrap_or_default()
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
