use crate::{
    AbstractVal,
    CallVal,
    ChangeVal,
    EquivVal,
    InverseVal,
    PairVal,
    ctx::ref1::CtxMeta,
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
pub(crate) struct Id;

pub(crate) const ID: &str = "id";

impl<T> Transformer<T, T> for Id {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: T) -> T
    where Ctx: CtxMeta<'a> {
        input
    }
}

impl ByVal<Val> for Id {
    fn transform_default<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        input
    }

    fn transform_symbol<'a, Ctx>(&self, _ctx: Ctx, symbol: Symbol) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Symbol(symbol)
    }

    fn transform_pair<'a, Ctx>(&self, _ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Pair(pair)
    }

    fn transform_change<'a, Ctx>(&self, _ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Change(change)
    }

    fn transform_call<'a, Ctx>(&self, _ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Call(call)
    }

    fn transform_equiv<'a, Ctx>(&self, _ctx: Ctx, equiv: EquivVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Equiv(equiv)
    }

    fn transform_inverse<'a, Ctx>(&self, _ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Inverse(inverse)
    }

    fn transform_abstract<'a, Ctx>(&self, _ctx: Ctx, abstract1: AbstractVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Abstract(abstract1)
    }

    fn transform_list<'a, Ctx>(&self, _ctx: Ctx, list: ListVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::List(list)
    }

    fn transform_map<'a, Ctx>(&self, _ctx: Ctx, map: MapVal) -> Val
    where Ctx: CtxMeta<'a> {
        Val::Map(map)
    }
}
