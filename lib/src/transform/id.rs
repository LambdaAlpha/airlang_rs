use crate::{
    call::Call,
    pair::Pair,
    reverse::Reverse,
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
};

#[derive(Copy, Clone)]
pub(crate) struct Id;

impl<Ctx> Transformer<Ctx, Val, Val> for Id {
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Id {
    fn transform_default(&self, _ctx: &mut Ctx, input: Val) -> Val {
        input
    }

    fn transform_symbol(&self, _ctx: &mut Ctx, s: Symbol) -> Val {
        Val::Symbol(s)
    }

    fn transform_pair(&self, _ctx: &mut Ctx, first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn transform_list(&self, _ctx: &mut Ctx, list: ListVal) -> Val {
        Val::List(list)
    }

    fn transform_map(&self, _ctx: &mut Ctx, map: MapVal) -> Val {
        Val::Map(map)
    }

    fn transform_call(&self, _ctx: &mut Ctx, func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn transform_reverse(&self, _ctx: &mut Ctx, func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}
