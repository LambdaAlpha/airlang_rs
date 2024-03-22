use crate::{
    call::Call,
    pair::Pair,
    reverse::Reverse,
    symbol::Symbol,
    transformer::{
        input::{
            ByRef,
            ByVal,
        },
        DefaultByRef,
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
    fn transform_atoms(&self, _ctx: &mut Ctx, input: Val) -> Val {
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

#[derive(Copy, Clone)]
pub(crate) struct IdByRef;

impl<'a, Ctx> Transformer<Ctx, &'a Val, Val> for IdByRef {
    fn transform(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::transform_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for IdByRef {
    fn transform_atoms(&self, _ctx: &mut Ctx, input: &'a Val) -> Val {
        input.clone()
    }

    fn transform_symbol(&self, _ctx: &mut Ctx, s: &'a Symbol) -> Val {
        Val::Symbol(s.clone())
    }

    fn transform_pair(&self, _ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        Val::Pair(Box::new(Pair::new(first.clone(), second.clone())))
    }

    fn transform_list(&self, _ctx: &mut Ctx, list: &'a ListVal) -> Val {
        Val::List(list.clone())
    }

    fn transform_map(&self, _ctx: &mut Ctx, map: &'a MapVal) -> Val {
        Val::Map(map.clone())
    }

    fn transform_call(&self, _ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        Val::Call(Box::new(Call::new(func.clone(), input.clone())))
    }

    fn transform_reverse(&self, _ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func.clone(), output.clone())))
    }
}
