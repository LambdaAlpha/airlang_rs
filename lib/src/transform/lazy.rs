use crate::{
    ctx_access::CtxAccessor,
    symbol::Symbol,
    transform::{
        eval::Eval,
        id::Id,
    },
    transformer::{
        input::ByVal,
        DefaultByVal,
        ValBuilder,
    },
    val::{
        list::ListVal,
        map::MapVal,
        Val,
    },
    Transformer,
};

#[derive(Copy, Clone)]
pub(crate) struct Lazy;

impl<Ctx> Transformer<Ctx, Val, Val> for Lazy
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Lazy
where
    Ctx: CtxAccessor,
{
    fn transform_default(&self, ctx: &mut Ctx, input: Val) -> Val {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        Id.transform_symbol(ctx, s)
    }

    fn transform_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::transform_pair(self, ctx, first, second, ValBuilder)
    }

    fn transform_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::transform_list(self, ctx, list, ValBuilder)
    }

    fn transform_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        DefaultByVal::transform_map(self, ctx, map, ValBuilder)
    }

    fn transform_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        if func.is_unit() {
            return Eval.transform(ctx, input);
        }
        DefaultByVal::transform_call(self, ctx, func, input, ValBuilder)
    }

    fn transform_ask(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultByVal::transform_ask(self, ctx, func, output, ValBuilder)
    }
}
