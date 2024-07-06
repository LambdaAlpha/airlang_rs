use crate::{
    ctx::{
        mut1::MutFnCtx,
        ref1::CtxMeta,
    },
    list::List,
    map::Map,
    pair::Pair,
    transform::{
        eval::Eval,
        id::Id,
        Transform,
    },
    transformer::{
        input::ByVal,
        Transformer,
    },
    ListVal,
    MapVal,
    PairVal,
    Val,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Mode {
    pub default: Transform,
    pub specialized: Option<Box<ValMode>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ValMode {
    pub pair: PairMode,
    pub list: ListMode,
    pub map: MapMode,
}

pub type PairMode = Pair<Mode, Mode>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMode {
    All(Mode),
    Some(List<ListItemMode>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListItemMode {
    pub mode: Mode,
    pub ellipsis: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapMode {
    All(PairMode),
    Some(Map<Val, Mode>),
}

impl Default for ListMode {
    fn default() -> Self {
        ListMode::All(Default::default())
    }
}

impl Default for MapMode {
    fn default() -> Self {
        MapMode::All(Default::default())
    }
}

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
            val => val,
        }
    }
}

impl Transformer<PairVal, Val> for PairMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, input: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let input = Pair::from(input);
        let first = self.first.transform(ctx.reborrow(), input.first);
        let second = self.second.transform(ctx, input.second);
        let pair = Pair::new(first, second);
        Val::Pair(pair.into())
    }
}

impl Transformer<ListVal, Val> for ListMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let val_list = List::from(val_list);
        match self {
            ListMode::All(mode) => {
                let list: List<Val> = val_list
                    .into_iter()
                    .map(|val| mode.transform(ctx.reborrow(), val))
                    .collect();
                Val::List(list.into())
            }
            ListMode::Some(mode_list) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(mode) = mode_iter.next() {
                    if mode.ellipsis {
                        let name_len = mode_iter.len();
                        let val_len = val_iter.len();
                        if val_len > name_len {
                            for _ in 0..(val_len - name_len) {
                                let val = val_iter.next().unwrap();
                                let val = mode.mode.transform(ctx.reborrow(), val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.mode.transform(ctx.reborrow(), val);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(Eval.transform(ctx.reborrow(), val));
                }
                Val::List(List::from(list).into())
            }
        }
    }
}

impl Transformer<MapVal, Val> for MapMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let val_map = Map::from(val_map);
        match self {
            MapMode::All(mode) => {
                let map: Map<Val, Val> = val_map
                    .into_iter()
                    .map(|(k, v)| {
                        let k = mode.first.transform(ctx.reborrow(), k);
                        let v = mode.second.transform(ctx.reborrow(), v);
                        (k, v)
                    })
                    .collect();
                Val::Map(map.into())
            }
            MapMode::Some(mode_map) => {
                let map: Map<Val, Val> = val_map
                    .into_iter()
                    .map(|(k, v)| {
                        let v = if let Some(mode) = mode_map.get(&k) {
                            mode.transform(ctx.reborrow(), v)
                        } else {
                            Eval.transform(ctx.reborrow(), v)
                        };
                        let k = Id.transform(ctx.reborrow(), k);
                        (k, v)
                    })
                    .collect();
                Val::Map(map.into())
            }
        }
    }
}

impl Mode {
    pub fn apply(&self, ctx: MutFnCtx, val: Val) -> Val {
        self.transform(ctx, val)
    }
}

pub(crate) mod repr;
