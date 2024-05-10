use crate::{
    ask::Ask,
    call::Call,
    ctx_access::CtxAccessor,
    list::List,
    map::Map,
    pair::Pair,
    problem::solve,
    transform::{
        eval::Eval,
        id::Id,
        Transform,
    },
    transformer::{
        input::ByVal,
        output::OutputBuilder,
        Transformer,
        ValBuilder,
    },
    AskVal,
    CallVal,
    CtxForMutableFn,
    ListVal,
    MapVal,
    PairVal,
    Symbol,
    Val,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Mode {
    Predefined(Transform),
    Custom(Box<ValMode>),
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ValMode {
    pub symbol: SymbolMode,
    pub pair: Box<PairMode>,
    pub list: Box<ListMode>,
    pub map: Box<MapMode>,
    pub call: Box<CallMode>,
    pub ask: Box<AskMode>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SymbolMode {
    #[default]
    Eval,
    Id,
}

pub type PairMode = Pair<Mode, Mode>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ListMode {
    All(Mode),
    Some(List<ListItemMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListItemMode {
    pub mode: Mode,
    pub ellipsis: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MapMode {
    All(PairMode),
    Some(Map<Val, Mode>),
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CallMode {
    #[default]
    Eval,
    Struct(Call<Mode, Mode>),
    Dependent(CallDepMode),
}

// decide transform mode of input by the type of function
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct CallDepMode {
    pub unit: Mode,
    pub bool: Mode,
    pub int: Mode,
    pub float: Mode,
    pub bytes: Mode,
    pub string: Mode,
    pub symbol: Mode,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AskMode {
    #[default]
    Eval,
    Struct(Ask<Mode, Mode>),
    Dependent(AskDepMode),
}

// decide transform mode of output by the type of function
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct AskDepMode {
    pub unit: Mode,
    pub bool: Mode,
    pub int: Mode,
    pub float: Mode,
    pub bytes: Mode,
    pub string: Mode,
    pub symbol: Mode,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Predefined(Default::default())
    }
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
        Ctx: CtxAccessor<'a>,
    {
        match self {
            Mode::Predefined(mode) => mode.transform(ctx, input),
            Mode::Custom(mode) => mode.transform(ctx, input),
        }
    }
}

impl Transformer<Val, Val> for ValMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        match input {
            Val::Symbol(s) => self.symbol.transform(ctx, s),
            Val::Pair(pair) => self.pair.transform(ctx, *pair),
            Val::Call(call) => self.call.transform(ctx, *call),
            Val::Ask(ask) => self.ask.transform(ctx, *ask),
            Val::List(list) => self.list.transform(ctx, list),
            Val::Map(map) => self.map.transform(ctx, map),
            val => val,
        }
    }
}

impl Transformer<Symbol, Val> for SymbolMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Symbol) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        match self {
            SymbolMode::Eval => Eval.transform_symbol(ctx, input),
            SymbolMode::Id => Id.transform_symbol(ctx, input),
        }
    }
}

impl Transformer<PairVal, Val> for PairMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, input: PairVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        let first = self.first.transform(ctx.reborrow(), input.first);
        let second = self.second.transform(ctx, input.second);
        ValBuilder.from_pair(first, second)
    }
}

impl Transformer<ListVal, Val> for ListMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_list: ListVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        match self {
            ListMode::All(mode) => {
                let list = val_list
                    .into_iter()
                    .map(|val| mode.transform(ctx.reborrow(), val))
                    .collect();
                Val::List(list)
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
                ValBuilder.from_list(list.into_iter())
            }
        }
    }
}

impl Transformer<MapVal, Val> for MapMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_map: MapVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        match self {
            MapMode::All(mode) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.transform(ctx.reborrow(), k);
                    let v = mode.second.transform(ctx.reborrow(), v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            MapMode::Some(mode_map) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(&k) {
                        mode.transform(ctx.reborrow(), v)
                    } else {
                        Eval.transform(ctx.reborrow(), v)
                    };
                    let k = Id.transform(ctx.reborrow(), k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
        }
    }
}

impl Transformer<CallVal, Val> for CallMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        match self {
            CallMode::Eval => Eval.transform_call(ctx, call.func, call.input),
            CallMode::Struct(mode) => mode.transform(ctx, call),
            CallMode::Dependent(mode) => mode.transform(ctx, call),
        }
    }
}

impl Transformer<CallVal, Val> for CallDepMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        let func = Eval.transform(ctx.reborrow(), call.func);
        let transformer = match func {
            Val::Func(f) => {
                let input = f.input_mode.transform(ctx.reborrow(), call.input);
                return f.transform(ctx, input);
            }
            Val::Unit(_) => &self.unit,
            Val::Bool(_) => &self.bool,
            Val::Int(_) => &self.int,
            Val::Float(_) => &self.float,
            Val::Bytes(_) => &self.bytes,
            Val::String(_) => &self.string,
            Val::Symbol(_) => &self.symbol,
            _ => {
                let input = Eval.transform(ctx, call.input);
                return ValBuilder.from_call(func, input);
            }
        };
        let input = transformer.transform(ctx, call.input);
        ValBuilder.from_call(func, input)
    }
}

impl Transformer<CallVal, Val> for Call<Mode, Mode> {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        let func = self.func.transform(ctx.reborrow(), call.func);
        let input = self.input.transform(ctx, call.input);
        ValBuilder.from_call(func, input)
    }
}

impl Transformer<AskVal, Val> for AskMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: AskVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        match self {
            AskMode::Eval => Eval.transform_ask(ctx, input.func, input.output),
            AskMode::Struct(ask) => ask.transform(ctx, input),
            AskMode::Dependent(ask) => ask.transform(ctx, input),
        }
    }
}

impl Transformer<AskVal, Val> for AskDepMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        let func = Eval.transform(ctx.reborrow(), ask.func);
        let transformer = match func {
            Val::Func(f) => {
                let output = f.output_mode.transform(ctx.reborrow(), ask.output);
                return solve(ctx, f, output);
            }
            Val::Unit(_) => &self.unit,
            Val::Bool(_) => &self.bool,
            Val::Int(_) => &self.int,
            Val::Float(_) => &self.float,
            Val::Bytes(_) => &self.bytes,
            Val::String(_) => &self.string,
            Val::Symbol(_) => &self.symbol,
            _ => {
                let output = Eval.transform(ctx, ask.output);
                return ValBuilder.from_ask(func, output);
            }
        };
        let output = transformer.transform(ctx, ask.output);
        ValBuilder.from_ask(func, output)
    }
}

impl Transformer<AskVal, Val> for Ask<Mode, Mode> {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        let func = self.func.transform(ctx.reborrow(), ask.func);
        let output = self.output.transform(ctx, ask.output);
        ValBuilder.from_ask(func, output)
    }
}

impl Mode {
    pub fn apply(&self, ctx: CtxForMutableFn, val: Val) -> Val {
        self.transform(ctx, val)
    }
}

pub(crate) mod repr;
