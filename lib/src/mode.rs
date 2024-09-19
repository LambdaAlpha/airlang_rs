use crate::{
    ctx::{
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    transformer::{
        input::ByVal,
        Transformer,
    },
    BasicMode,
    ListMode,
    MapMode,
    PairMode,
    Val,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Mode {
    pub default: BasicMode,
    pub specialized: Option<Box<ValMode>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ValMode {
    pub pair: PairMode,
    pub list: ListMode,
    pub map: MapMode,
}

pub(crate) const SYMBOL_READ_PREFIX: char = '$';
pub(crate) const SYMBOL_MOVE_PREFIX: char = '&';

impl Transformer<Val, Val> for Mode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let Some(val_mode) = &self.specialized else {
            return self.default.transform(ctx, input);
        };
        match input {
            Val::Symbol(s) => self.default.transform_symbol(ctx, s),
            Val::Call(call) => self.default.transform_call(ctx, call),
            Val::Ask(ask) => self.default.transform_ask(ctx, ask),
            Val::Pair(pair) => val_mode.pair.transform(ctx, pair),
            Val::List(list) => val_mode.list.transform(ctx, list),
            Val::Map(map) => val_mode.map.transform(ctx, map),
            val => self.default.transform(ctx, val),
        }
    }
}

impl Mode {
    pub fn apply(&self, ctx: MutFnCtx, val: Val) -> Val {
        self.transform(ctx, val)
    }
}

pub(crate) mod id;

pub(crate) mod form;

pub(crate) mod eval;

pub(crate) mod basic;

pub(crate) mod pair;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod repr;
