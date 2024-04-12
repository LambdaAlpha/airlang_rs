use crate::{
    call::Call,
    ctx_access::CtxAccessor,
    list::List,
    map::Map,
    pair::Pair,
    problem::solve,
    reverse::Reverse,
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
    CallVal,
    CtxForMutableFn,
    ListVal,
    MapVal,
    PairVal,
    ReverseVal,
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
    pub reverse: Box<ReverseMode>,
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
pub enum ReverseMode {
    #[default]
    Eval,
    Struct(Reverse<Mode, Mode>),
    Dependent(ReverseDepMode),
}

// decide transform mode of output by the type of function
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ReverseDepMode {
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

impl<Ctx> Transformer<Ctx, Val, Val> for Mode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            Mode::Predefined(mode) => mode.transform(ctx, input),
            Mode::Custom(mode) => mode.transform(ctx, input),
        }
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for ValMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match input {
            Val::Symbol(s) => self.symbol.transform(ctx, s),
            Val::Pair(pair) => self.pair.transform(ctx, *pair),
            Val::Call(call) => self.call.transform(ctx, *call),
            Val::Reverse(reverse) => self.reverse.transform(ctx, *reverse),
            Val::List(list) => self.list.transform(ctx, list),
            Val::Map(map) => self.map.transform(ctx, map),
            val => val,
        }
    }
}

impl<Ctx> Transformer<Ctx, Symbol, Val> for SymbolMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Symbol) -> Val {
        match self {
            SymbolMode::Eval => Eval.transform_symbol(ctx, input),
            SymbolMode::Id => Id.transform_symbol(ctx, input),
        }
    }
}

impl<Ctx> Transformer<Ctx, PairVal, Val> for PairMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: PairVal) -> Val {
        let first = self.first.transform(ctx, input.first);
        let second = self.second.transform(ctx, input.second);
        ValBuilder.from_pair(first, second)
    }
}

impl<Ctx> Transformer<Ctx, ListVal, Val> for ListMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, val_list: ListVal) -> Val {
        match self {
            ListMode::All(mode) => {
                let list = val_list
                    .into_iter()
                    .map(|val| mode.transform(ctx, val))
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
                                let val = mode.mode.transform(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.mode.transform(ctx, val);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(Eval.transform(ctx, val));
                }
                ValBuilder.from_list(list.into_iter())
            }
        }
    }
}

impl<Ctx> Transformer<Ctx, MapVal, Val> for MapMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, val_map: MapVal) -> Val {
        match self {
            MapMode::All(mode) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.transform(ctx, k);
                    let v = mode.second.transform(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            MapMode::Some(mode_map) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(&k) {
                        mode.transform(ctx, v)
                    } else {
                        Eval.transform(ctx, v)
                    };
                    let k = Id.transform(ctx, k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
        }
    }
}

impl<Ctx> Transformer<Ctx, CallVal, Val> for CallMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, call: CallVal) -> Val {
        match self {
            CallMode::Eval => Eval.transform_call(ctx, call.func, call.input),
            CallMode::Struct(mode) => mode.transform(ctx, call),
            CallMode::Dependent(mode) => mode.transform(ctx, call),
        }
    }
}

impl<Ctx> Transformer<Ctx, CallVal, Val> for CallDepMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, call: CallVal) -> Val {
        let func = Eval.transform(ctx, call.func);
        let transformer = match func {
            Val::Func(f) => {
                let input = f.input_mode.transform(ctx, call.input);
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

impl<Ctx> Transformer<Ctx, CallVal, Val> for Call<Mode, Mode>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, call: CallVal) -> Val {
        let func = self.func.transform(ctx, call.func);
        let input = self.input.transform(ctx, call.input);
        ValBuilder.from_call(func, input)
    }
}

impl<Ctx> Transformer<Ctx, ReverseVal, Val> for ReverseMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: ReverseVal) -> Val {
        match self {
            ReverseMode::Eval => Eval.transform_reverse(ctx, input.func, input.output),
            ReverseMode::Struct(reverse) => reverse.transform(ctx, input),
            ReverseMode::Dependent(reverse) => reverse.transform(ctx, input),
        }
    }
}

impl<Ctx> Transformer<Ctx, ReverseVal, Val> for ReverseDepMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, reverse: ReverseVal) -> Val {
        let func = Eval.transform(ctx, reverse.func);
        let transformer = match func {
            Val::Func(f) => {
                let output = f.output_mode.transform(ctx, reverse.output);
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
                let output = Eval.transform(ctx, reverse.output);
                return ValBuilder.from_reverse(func, output);
            }
        };
        let output = transformer.transform(ctx, reverse.output);
        ValBuilder.from_reverse(func, output)
    }
}

impl<Ctx> Transformer<Ctx, ReverseVal, Val> for Reverse<Mode, Mode>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, reverse: ReverseVal) -> Val {
        let func = self.func.transform(ctx, reverse.func);
        let output = self.output.transform(ctx, reverse.output);
        ValBuilder.from_reverse(func, output)
    }
}

impl Mode {
    pub fn apply(&self, mut ctx: CtxForMutableFn, val: Val) -> Val {
        self.transform(&mut ctx, val)
    }
}

pub(crate) mod repr;
