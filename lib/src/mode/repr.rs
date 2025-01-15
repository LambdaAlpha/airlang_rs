use crate::{
    Abstract,
    Ask,
    Bit,
    Call,
    Id,
    List,
    ListVal,
    Map,
    MapVal,
    Pair,
    PairMode,
    PrefixMode,
    PrimMode,
    Symbol,
    SymbolMode,
    UniMode,
    Val,
    mode::{
        Mode,
        abstract1::AbstractMode,
        ask::AskMode,
        call::CallMode,
        comp::CompMode,
        eval::{
            Eval,
            EvalMode,
        },
        form::{
            Form,
            FormMode,
        },
        list::ListMode,
        map::MapMode,
        symbol::{
            LITERAL_STR,
            MOVE_STR,
            REF_STR,
        },
        united::{
            EVAL,
            EVAL_LITERAL,
            EVAL_MOVE,
            EVAL_REF,
            FORM,
            FORM_LITERAL,
            FORM_MOVE,
            FORM_REF,
            ID,
        },
    },
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        ABSTRACT,
        ASK,
        CALL,
        LIST,
        MAP,
        PAIR,
        SYMBOL,
    },
};

pub(crate) trait ParseMode<T>: Sized + Clone {
    fn parse(mode: T, default: UniMode) -> Option<Self>;
}

pub(crate) trait GenerateMode<T> {
    fn generate(&self, default: UniMode) -> T;
}

pub(crate) fn parse(mode: Val) -> Option<Mode> {
    Mode::parse(mode, UniMode::default())
}

pub(crate) fn generate(mode: &Mode) -> Val {
    mode.generate(UniMode::default())
}

const DEFAULT: &str = "default";
const PRIMITIVE: &str = "primitive";

impl ParseMode<Val> for Mode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(symbol) => Some(Self::from(UniMode::parse(symbol, default)?)),
            Val::Map(mut map) => {
                let primitive = match map_remove(&mut map, PRIMITIVE) {
                    Val::Unit(_) => false,
                    Val::Bit(b) => b.bool(),
                    _ => return None,
                };
                let mode = if primitive {
                    Mode::Prim(PrimMode::parse(map, default)?)
                } else {
                    Mode::Comp(Box::new(CompMode::parse(map, default)?))
                };
                Some(mode)
            }
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for Mode {
    fn parse(mode: Symbol, default: UniMode) -> Option<Self> {
        let mode = UniMode::parse(mode, default)?;
        Some(Mode::from(mode))
    }
}

impl GenerateMode<Val> for Mode {
    fn generate(&self, default: UniMode) -> Val {
        match self {
            Mode::Uni(mode) => Val::Symbol(mode.generate(default)),
            Mode::Prim(mode) => {
                let mut map = mode.generate(default);
                map.insert(symbol(PRIMITIVE), Val::Bit(Bit::true1()));
                Val::Map(map)
            }
            Mode::Comp(mode) => {
                let mode = mode.generate(default);
                Val::Map(mode)
            }
        }
    }
}

impl ParseMode<Val> for UniMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(default),
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for UniMode {
    fn parse(mode: Symbol, _default: UniMode) -> Option<Self> {
        let mode = match &*mode {
            ID => UniMode::Id(Id),
            FORM => UniMode::Form(Form::new(PrefixMode::default())),
            EVAL => UniMode::Eval(Eval::new(PrefixMode::default())),
            LITERAL_STR => UniMode::Eval(Eval::new(PrefixMode::Literal)),
            REF_STR => UniMode::Eval(Eval::new(PrefixMode::Ref)),
            MOVE_STR => UniMode::Eval(Eval::new(PrefixMode::Move)),
            FORM_LITERAL => UniMode::Form(Form::new(PrefixMode::Literal)),
            FORM_REF => UniMode::Form(Form::new(PrefixMode::Ref)),
            FORM_MOVE => UniMode::Form(Form::new(PrefixMode::Move)),
            EVAL_LITERAL => UniMode::Eval(Eval::new(PrefixMode::Literal)),
            EVAL_REF => UniMode::Eval(Eval::new(PrefixMode::Ref)),
            EVAL_MOVE => UniMode::Eval(Eval::new(PrefixMode::Move)),
            _ => return None,
        };
        Some(mode)
    }
}

impl GenerateMode<Symbol> for UniMode {
    fn generate(&self, _default: UniMode) -> Symbol {
        let s = match self {
            UniMode::Id(_) => ID,
            UniMode::Form(mode) => match mode.prefix_mode() {
                PrefixMode::Literal => FORM_LITERAL,
                PrefixMode::Ref => FORM_REF,
                PrefixMode::Move => FORM_MOVE,
            },
            UniMode::Eval(mode) => match mode.prefix_mode() {
                PrefixMode::Literal => EVAL_LITERAL,
                PrefixMode::Ref => EVAL_REF,
                PrefixMode::Move => EVAL_MOVE,
            },
        };
        Symbol::from_str(s)
    }
}

impl ParseMode<Val> for FormMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for FormMode {
    fn parse(mode: Symbol, _default: UniMode) -> Option<Self> {
        match &*mode {
            ID => Some(FormMode::Id),
            FORM => Some(FormMode::Form),
            _ => None,
        }
    }
}

impl GenerateMode<Symbol> for FormMode {
    fn generate(&self, _default: UniMode) -> Symbol {
        let s = match self {
            FormMode::Id => ID,
            FormMode::Form => FORM,
        };
        Symbol::from_str(s)
    }
}

impl ParseMode<Val> for EvalMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for EvalMode {
    fn parse(mode: Symbol, _default: UniMode) -> Option<Self> {
        match &*mode {
            ID => Some(EvalMode::Id),
            FORM => Some(EvalMode::Form),
            EVAL => Some(EvalMode::Eval),
            _ => None,
        }
    }
}

impl GenerateMode<Symbol> for EvalMode {
    fn generate(&self, _default: UniMode) -> Symbol {
        let s = match self {
            EvalMode::Id => ID,
            EvalMode::Form => FORM,
            EvalMode::Eval => EVAL,
        };
        Symbol::from_str(s)
    }
}

impl ParseMode<MapVal> for CompMode {
    fn parse(mut map: MapVal, default: UniMode) -> Option<Self> {
        let default = UniMode::parse(map_remove(&mut map, DEFAULT), default)?;
        let symbol = SymbolMode::parse(map_remove(&mut map, SYMBOL), default)?;
        let pair = PairMode::parse(map_remove(&mut map, PAIR), default)?;
        let abstract1 = AbstractMode::parse(map_remove(&mut map, ABSTRACT), default)?;
        let call = CallMode::parse(map_remove(&mut map, CALL), default)?;
        let ask = AskMode::parse(map_remove(&mut map, ASK), default)?;
        let list = ListMode::parse(map_remove(&mut map, LIST), default)?;
        let map = MapMode::parse(map_remove(&mut map, MAP), default)?;
        Some(CompMode {
            symbol,
            pair,
            abstract1,
            call,
            ask,
            list,
            map,
        })
    }
}

impl GenerateMode<MapVal> for CompMode {
    fn generate(&self, default: UniMode) -> MapVal {
        let mut map = Map::<Val, Val>::default();
        if SymbolMode::from(default) != self.symbol {
            map.insert(symbol(SYMBOL), Val::Symbol(self.symbol.generate(default)));
        }
        if PairMode::from(default) != self.pair {
            map.insert(symbol(PAIR), self.pair.generate(default));
        }
        if AbstractMode::from(default) != self.abstract1 {
            map.insert(symbol(ABSTRACT), self.abstract1.generate(default));
        }
        if CallMode::from(default) != self.call {
            map.insert(symbol(CALL), self.call.generate(default));
        }
        if AskMode::from(default) != self.ask {
            map.insert(symbol(ASK), self.ask.generate(default));
        }
        if ListMode::from(default) != self.list {
            map.insert(symbol(LIST), self.list.generate(default));
        }
        if MapMode::from(default) != self.map {
            map.insert(symbol(MAP), self.map.generate(default));
        }
        map.into()
    }
}

impl ParseMode<MapVal> for PrimMode {
    fn parse(mut map: MapVal, default: UniMode) -> Option<Self> {
        let default = UniMode::parse(map_remove(&mut map, DEFAULT), default)?;
        let symbol = SymbolMode::parse(map_remove(&mut map, SYMBOL), default)?;
        let pair = FormMode::parse(map_remove(&mut map, PAIR), default)?;
        let abstract1 = EvalMode::parse(map_remove(&mut map, ABSTRACT), default)?;
        let call = EvalMode::parse(map_remove(&mut map, CALL), default)?;
        let ask = EvalMode::parse(map_remove(&mut map, ASK), default)?;
        let list = FormMode::parse(map_remove(&mut map, LIST), default)?;
        let map = FormMode::parse(map_remove(&mut map, MAP), default)?;
        Some(PrimMode {
            symbol,
            pair,
            abstract1,
            call,
            ask,
            list,
            map,
        })
    }
}

impl GenerateMode<MapVal> for PrimMode {
    fn generate(&self, default: UniMode) -> MapVal {
        let mut map = Map::<Val, Val>::default();
        if SymbolMode::from(default) != self.symbol {
            map.insert(symbol(SYMBOL), Val::Symbol(self.symbol.generate(default)));
        }
        if FormMode::from(default) != self.pair {
            map.insert(symbol(PAIR), Val::Symbol(self.pair.generate(default)));
        }
        if EvalMode::from(default) != self.abstract1 {
            map.insert(
                symbol(ABSTRACT),
                Val::Symbol(self.abstract1.generate(default)),
            );
        }
        if EvalMode::from(default) != self.call {
            map.insert(symbol(CALL), Val::Symbol(self.call.generate(default)));
        }
        if EvalMode::from(default) != self.ask {
            map.insert(symbol(ASK), Val::Symbol(self.ask.generate(default)));
        }
        if FormMode::from(default) != self.list {
            map.insert(symbol(LIST), Val::Symbol(self.list.generate(default)));
        }
        if FormMode::from(default) != self.map {
            map.insert(symbol(MAP), Val::Symbol(self.map.generate(default)));
        }
        map.into()
    }
}

impl ParseMode<Val> for SymbolMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for SymbolMode {
    fn parse(mode: Symbol, _default: UniMode) -> Option<Self> {
        let mode = match &*mode {
            ID => SymbolMode::Id(Id),
            LITERAL_STR => SymbolMode::Form(PrefixMode::Literal),
            REF_STR => SymbolMode::Form(PrefixMode::Ref),
            MOVE_STR => SymbolMode::Form(PrefixMode::Move),
            _ => return None,
        };
        Some(mode)
    }
}

impl GenerateMode<Symbol> for SymbolMode {
    fn generate(&self, _default: UniMode) -> Symbol {
        let s = match self {
            SymbolMode::Id(_) => ID,
            SymbolMode::Form(mode) => match mode {
                PrefixMode::Literal => LITERAL_STR,
                PrefixMode::Ref => REF_STR,
                PrefixMode::Move => MOVE_STR,
            },
        };
        Symbol::from_str(s)
    }
}

impl ParseMode<Val> for PairMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let first = Mode::parse(pair.first, default)?;
                let second = Mode::parse(pair.second, default)?;
                Some(PairMode::Form(Pair::new(first, second)))
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for PairMode {
    fn generate(&self, default: UniMode) -> Val {
        match self {
            PairMode::Id(_) => symbol(ID),
            PairMode::Form(mode) => {
                let first = Mode::generate(&mode.first, default);
                let second = Mode::generate(&mode.second, default);
                Val::Pair(Pair::new(first, second).into())
            }
        }
    }
}

impl ParseMode<Val> for AbstractMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let func = Mode::parse(pair.first, default)?;
                let input = Mode::parse(pair.second, default)?;
                Some(AbstractMode::Form(Abstract::new(func, input)))
            }
            Val::Abstract(abstract1) => {
                let abstract1 = Abstract::from(abstract1);
                let func = Mode::parse(abstract1.func, default)?;
                let input = Mode::parse(abstract1.input, default)?;
                Some(AbstractMode::Eval(Abstract::new(func, input)))
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for AbstractMode {
    fn generate(&self, default: UniMode) -> Val {
        match self {
            AbstractMode::Id(_) => symbol(ID),
            AbstractMode::Form(mode) => {
                let func = Mode::generate(&mode.func, default);
                let input = Mode::generate(&mode.input, default);
                Val::Pair(Pair::new(func, input).into())
            }
            AbstractMode::Eval(mode) => {
                let func = Mode::generate(&mode.func, default);
                let input = Mode::generate(&mode.input, default);
                Val::Abstract(Abstract::new(func, input).into())
            }
        }
    }
}

impl ParseMode<Val> for CallMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let func = Mode::parse(pair.first, default)?;
                let input = Mode::parse(pair.second, default)?;
                Some(CallMode::Form(Call::new(func, input)))
            }
            Val::Call(call) => {
                let call = Call::from(call);
                let func = Mode::parse(call.func, default)?;
                let input = Mode::parse(call.input, default)?;
                Some(CallMode::Eval(Call::new(func, input)))
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for CallMode {
    fn generate(&self, default: UniMode) -> Val {
        match self {
            CallMode::Id(_) => symbol(ID),
            CallMode::Form(mode) => {
                let func = Mode::generate(&mode.func, default);
                let input = Mode::generate(&mode.input, default);
                Val::Pair(Pair::new(func, input).into())
            }
            CallMode::Eval(mode) => {
                let func = Mode::generate(&mode.func, default);
                let input = Mode::generate(&mode.input, default);
                Val::Call(Call::new(func, input).into())
            }
        }
    }
}

impl ParseMode<Val> for AskMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let func = Mode::parse(pair.first, default)?;
                let output = Mode::parse(pair.second, default)?;
                Some(AskMode::Form(Ask::new(func, output)))
            }
            Val::Ask(ask) => {
                let ask = Ask::from(ask);
                let func = Mode::parse(ask.func, default)?;
                let output = Mode::parse(ask.output, default)?;
                Some(AskMode::Eval(Ask::new(func, output)))
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for AskMode {
    fn generate(&self, default: UniMode) -> Val {
        match self {
            AskMode::Id(_) => symbol(ID),
            AskMode::Form(mode) => {
                let func = Mode::generate(&mode.func, default);
                let output = Mode::generate(&mode.output, default);
                Val::Pair(Pair::new(func, output).into())
            }
            AskMode::Eval(mode) => {
                let func = Mode::generate(&mode.func, default);
                let output = Mode::generate(&mode.output, default);
                Val::Ask(Ask::new(func, output).into())
            }
        }
    }
}

impl ParseMode<Val> for ListMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::List(head) => {
                let head = parse_list_head(head, default)?;
                let tail = Mode::from(default);
                Some(ListMode::Form { head, tail })
            }
            Val::Pair(head_tail) => {
                let head_tail = Pair::from(head_tail);
                let Val::List(head) = head_tail.first else {
                    return None;
                };
                let head = parse_list_head(head, default)?;
                let tail = Mode::parse(head_tail.second, default)?;
                Some(ListMode::Form { head, tail })
            }
            _ => None,
        }
    }
}

fn parse_list_head(head: ListVal, default: UniMode) -> Option<List<Mode>> {
    List::from(head)
        .into_iter()
        .map(|item| Mode::parse(item, default))
        .collect()
}

impl GenerateMode<Val> for ListMode {
    fn generate(&self, default: UniMode) -> Val {
        match self {
            ListMode::Id(_) => symbol(ID),
            ListMode::Form { head, tail } => {
                let head = generate_list_head(head, default);
                let tail_default = Mode::from(default) == *tail;
                if tail_default {
                    return head;
                }
                let tail = Mode::generate(tail, default);
                Val::Pair(Pair::new(head, tail).into())
            }
        }
    }
}

fn generate_list_head(head: &List<Mode>, default: UniMode) -> Val {
    let head: List<Val> = head
        .iter()
        .map(|item| Mode::generate(item, default))
        .collect();
    Val::List(head.into())
}

impl ParseMode<Val> for MapMode {
    fn parse(mode: Val, default: UniMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Map(some) => {
                let some = parse_map_some(some, default)?;
                let default = Mode::from(default);
                let else1 = Pair::new(default.clone(), default);
                Some(MapMode::Form { some, else1 })
            }
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    return None;
                };
                let some = parse_map_some(some, default)?;
                let else1 = parse_map_else(some_else.second, default)?;
                Some(MapMode::Form { some, else1 })
            }
            _ => None,
        }
    }
}

fn parse_map_some(some: MapVal, default: UniMode) -> Option<Map<Val, Mode>> {
    Map::from(some)
        .into_iter()
        .map(|(k, v)| {
            let mode = Mode::parse(v, default)?;
            Some((k, mode))
        })
        .collect()
}

fn parse_map_else(mode: Val, default: UniMode) -> Option<Pair<Mode, Mode>> {
    let mode = match mode {
        Val::Unit(_) => {
            let mode = Mode::from(default);
            Pair::new(mode.clone(), mode)
        }
        Val::Symbol(s) => {
            let mode = Mode::parse(s, default)?;
            Pair::new(mode.clone(), mode)
        }
        Val::Pair(else1) => {
            let else1 = Pair::from(else1);
            let key = Mode::parse(else1.first, default)?;
            let value = Mode::parse(else1.second, default)?;
            Pair::new(key, value)
        }
        _ => return None,
    };
    Some(mode)
}

impl GenerateMode<Val> for MapMode {
    fn generate(&self, default: UniMode) -> Val {
        match self {
            MapMode::Id(_) => symbol(ID),
            MapMode::Form { some, else1 } => {
                let some = generate_map_some(some, default);
                let default_mode = Mode::from(default);
                let else_default = default_mode == else1.first && default_mode == else1.second;
                if else_default {
                    return some;
                }
                let else1 = generate_map_else(&else1.first, &else1.second, default);
                Val::Pair(Pair::new(some, else1).into())
            }
        }
    }
}

fn generate_map_some<M: GenerateMode<Val>>(some: &Map<Val, M>, default: UniMode) -> Val {
    let some: Map<Val, Val> = some
        .iter()
        .map(|(k, v)| {
            let mode = M::generate(v, default);
            (k.clone(), mode)
        })
        .collect();
    Val::Map(some.into())
}

fn generate_map_else<M: GenerateMode<Val>>(k: &M, v: &M, default: UniMode) -> Val {
    let k = M::generate(k, default);
    let v = M::generate(v, default);
    Val::Pair(Pair::new(k, v).into())
}
