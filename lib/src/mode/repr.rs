use crate::{
    Ask,
    Bool,
    Call,
    Comment,
    List,
    ListVal,
    Map,
    MapVal,
    Pair,
    PairMode,
    PrimitiveMode,
    Symbol,
    SymbolMode,
    Unit,
    Val,
    mode::{
        Mode,
        ask::AskMode,
        call::CallMode,
        comment::CommentMode,
        composite::CompositeMode,
        list::ListMode,
        map::MapMode,
        primitive::{
            EVAL,
            FORM,
            ID,
        },
        recursive::SelfMode,
    },
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        ASK,
        CALL,
        COMMENT,
        LIST,
        MAP,
        PAIR,
        SYMBOL,
    },
};

pub(crate) trait ParseMode<T>: Sized + Clone {
    fn parse(mode: T, default: PrimitiveMode) -> Option<Self>;
}

pub(crate) trait GenerateMode<T> {
    fn generate(&self, default: PrimitiveMode) -> T;
}

pub(crate) fn parse(mode: Val) -> Option<Mode> {
    Mode::parse(mode, PrimitiveMode::default())
}

pub(crate) fn generate(mode: &Mode) -> Val {
    mode.generate(PrimitiveMode::default())
}

const DEFAULT: &str = "default";
const RECURSIVE: &str = "recursive";
const SELF: &str = "self";

impl ParseMode<Val> for Mode {
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(PrimitiveMode::parse(s, default)?)),
            Val::Map(mut map) => {
                let recursive = match map_remove(&mut map, RECURSIVE) {
                    Val::Unit(_) => false,
                    Val::Bool(b) => b.bool(),
                    _ => return None,
                };
                let mode = if recursive {
                    Mode::Recursive(CompositeMode::<SelfMode>::parse(map, default)?)
                } else {
                    Mode::Composite(Box::new(CompositeMode::<Mode>::parse(map, default)?))
                };
                Some(mode)
            }
            _ => None,
        }
    }
}

impl ParseMode<Unit> for Mode {
    fn parse(_: Unit, default: PrimitiveMode) -> Option<Self> {
        Some(Mode::from(default))
    }
}

impl ParseMode<Symbol> for Mode {
    fn parse(mode: Symbol, default: PrimitiveMode) -> Option<Self> {
        let mode = PrimitiveMode::parse(mode, default)?;
        Some(Mode::from(mode))
    }
}

impl GenerateMode<Val> for Mode {
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            Mode::Primitive(mode) => mode.generate(default),
            Mode::Recursive(mode) => {
                let mut map = mode.generate(default);
                map.insert(symbol(RECURSIVE), Val::Bool(Bool::t()));
                Val::Map(map)
            }
            Mode::Composite(mode) => {
                let mode = mode.generate(default);
                Val::Map(mode)
            }
        }
    }
}

impl ParseMode<Val> for SelfMode {
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(SelfMode::default()),
            Val::Symbol(s) => Some(Self::parse(s, default)?),
            _ => None,
        }
    }
}

impl ParseMode<Unit> for SelfMode {
    fn parse(_: Unit, _default: PrimitiveMode) -> Option<Self> {
        Some(SelfMode::Self1)
    }
}

impl ParseMode<Symbol> for SelfMode {
    fn parse(mode: Symbol, default: PrimitiveMode) -> Option<Self> {
        if &*mode == SELF {
            return Some(SelfMode::Self1);
        }
        let mode = PrimitiveMode::parse(mode, default)?;
        Some(Self::Primitive(mode))
    }
}

impl GenerateMode<Val> for SelfMode {
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            SelfMode::Self1 => symbol(SELF),
            SelfMode::Primitive(mode) => mode.generate(default),
        }
    }
}

impl ParseMode<Val> for PrimitiveMode {
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(default),
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for PrimitiveMode {
    fn parse(mode: Symbol, _default: PrimitiveMode) -> Option<Self> {
        let mode = match &*mode {
            ID => PrimitiveMode::Id,
            FORM => PrimitiveMode::Form,
            EVAL => PrimitiveMode::Eval,
            _ => return None,
        };
        Some(mode)
    }
}

impl GenerateMode<Val> for PrimitiveMode {
    fn generate(&self, _default: PrimitiveMode) -> Val {
        let s = match self {
            PrimitiveMode::Id => ID,
            PrimitiveMode::Form => FORM,
            PrimitiveMode::Eval => EVAL,
        };
        symbol(s)
    }
}

impl<M> ParseMode<MapVal> for CompositeMode<M>
where
    M: ParseMode<Val> + ParseMode<Unit> + ParseMode<Symbol>,
    PairMode<M>: From<PrimitiveMode>,
    CommentMode<M>: From<PrimitiveMode>,
    CallMode<M>: From<PrimitiveMode>,
    AskMode<M>: From<PrimitiveMode>,
    ListMode<M>: From<PrimitiveMode>,
    MapMode<M>: From<PrimitiveMode>,
{
    fn parse(mut map: MapVal, default: PrimitiveMode) -> Option<Self> {
        let default = PrimitiveMode::parse(map_remove(&mut map, DEFAULT), default)?;
        let symbol = SymbolMode::parse(map_remove(&mut map, SYMBOL), default)?;
        let pair = PairMode::parse(map_remove(&mut map, PAIR), default)?;
        let comment = CommentMode::parse(map_remove(&mut map, COMMENT), default)?;
        let call = CallMode::parse(map_remove(&mut map, CALL), default)?;
        let ask = AskMode::parse(map_remove(&mut map, ASK), default)?;
        let list = ListMode::parse(map_remove(&mut map, LIST), default)?;
        let map = MapMode::parse(map_remove(&mut map, MAP), default)?;
        Some(CompositeMode {
            symbol,
            pair,
            comment,
            call,
            ask,
            list,
            map,
        })
    }
}

impl<M> GenerateMode<MapVal> for CompositeMode<M>
where
    M: GenerateMode<Val> + ParseMode<Unit> + PartialEq,
    PairMode<M>: From<PrimitiveMode> + PartialEq,
    CommentMode<M>: From<PrimitiveMode> + PartialEq,
    CallMode<M>: From<PrimitiveMode> + PartialEq,
    AskMode<M>: From<PrimitiveMode> + PartialEq,
    ListMode<M>: From<PrimitiveMode> + PartialEq,
    MapMode<M>: From<PrimitiveMode> + PartialEq,
{
    fn generate(&self, default: PrimitiveMode) -> MapVal {
        let mut map = Map::<Val, Val>::default();
        if SymbolMode::from(default) != self.symbol {
            map.insert(symbol(SYMBOL), self.symbol.generate(default));
        }
        if PairMode::from(default) != self.pair {
            map.insert(symbol(PAIR), self.pair.generate(default));
        }
        if CommentMode::from(default) != self.comment {
            map.insert(symbol(COMMENT), self.comment.generate(default));
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

impl ParseMode<Val> for SymbolMode {
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        let mode = PrimitiveMode::parse(mode, default)?;
        Some(mode.into())
    }
}

impl GenerateMode<Val> for SymbolMode {
    fn generate(&self, default: PrimitiveMode) -> Val {
        PrimitiveMode::from(*self).generate(default)
    }
}

impl<M> ParseMode<Val> for PairMode<M>
where
    M: ParseMode<Val>,
    PairMode<M>: From<PrimitiveMode>,
{
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(PrimitiveMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let first = M::parse(pair.first, default)?;
                let second = M::parse(pair.second, default)?;
                Some(PairMode::Form(Pair::new(first, second)))
            }
            _ => None,
        }
    }
}

impl<M: GenerateMode<Val>> GenerateMode<Val> for PairMode<M> {
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            PairMode::Id => symbol(ID),
            PairMode::Form(mode) => {
                let first = M::generate(&mode.first, default);
                let second = M::generate(&mode.second, default);
                Val::Pair(Pair::new(first, second).into())
            }
        }
    }
}

impl<M> ParseMode<Val> for CommentMode<M>
where
    M: ParseMode<Val>,
    CommentMode<M>: From<PrimitiveMode>,
{
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(PrimitiveMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let meta = M::parse(pair.first, default)?;
                let value = M::parse(pair.second, default)?;
                Some(CommentMode::Form(Comment::new(meta, value)))
            }
            Val::Comment(comment) => {
                let comment = Comment::from(comment);
                let value = M::parse(comment.value, default)?;
                Some(CommentMode::Eval(value))
            }
            _ => None,
        }
    }
}

impl<M: GenerateMode<Val>> GenerateMode<Val> for CommentMode<M> {
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            CommentMode::Id => symbol(ID),
            CommentMode::Form(mode) => {
                let meta = M::generate(&mode.meta, default);
                let value = M::generate(&mode.value, default);
                Val::Pair(Pair::new(meta, value).into())
            }
            CommentMode::Eval(mode) => {
                let meta = Val::default();
                let value = M::generate(mode, default);
                Val::Comment(Comment::new(meta, value).into())
            }
        }
    }
}

impl<M> ParseMode<Val> for CallMode<M>
where
    M: ParseMode<Val>,
    CallMode<M>: From<PrimitiveMode>,
{
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(PrimitiveMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let func = M::parse(pair.first, default)?;
                let input = M::parse(pair.second, default)?;
                Some(CallMode::Form(Call::new(func, input)))
            }
            Val::Call(call) => {
                let call = Call::from(call);
                let func = M::parse(call.func, default)?;
                let input = M::parse(call.input, default)?;
                Some(CallMode::Eval(Call::new(func, input)))
            }
            _ => None,
        }
    }
}

impl<M: GenerateMode<Val>> GenerateMode<Val> for CallMode<M> {
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            CallMode::Id => symbol(ID),
            CallMode::Form(mode) => {
                let func = M::generate(&mode.func, default);
                let input = M::generate(&mode.input, default);
                Val::Pair(Pair::new(func, input).into())
            }
            CallMode::Eval(mode) => {
                let func = M::generate(&mode.func, default);
                let input = M::generate(&mode.input, default);
                Val::Call(Call::new(func, input).into())
            }
        }
    }
}

impl<M> ParseMode<Val> for AskMode<M>
where
    M: ParseMode<Val>,
    AskMode<M>: From<PrimitiveMode>,
{
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(PrimitiveMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let func = M::parse(pair.first, default)?;
                let output = M::parse(pair.second, default)?;
                Some(AskMode::Form(Ask::new(func, output)))
            }
            Val::Ask(ask) => {
                let ask = Ask::from(ask);
                let func = M::parse(ask.func, default)?;
                let output = M::parse(ask.output, default)?;
                Some(AskMode::Eval(Ask::new(func, output)))
            }
            _ => None,
        }
    }
}

impl<M: GenerateMode<Val>> GenerateMode<Val> for AskMode<M> {
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            AskMode::Id => symbol(ID),
            AskMode::Form(mode) => {
                let func = M::generate(&mode.func, default);
                let output = M::generate(&mode.output, default);
                Val::Pair(Pair::new(func, output).into())
            }
            AskMode::Eval(mode) => {
                let func = M::generate(&mode.func, default);
                let output = M::generate(&mode.output, default);
                Val::Ask(Ask::new(func, output).into())
            }
        }
    }
}

impl<M> ParseMode<Val> for ListMode<M>
where
    M: ParseMode<Val> + ParseMode<Unit>,
    ListMode<M>: From<PrimitiveMode>,
{
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(PrimitiveMode::parse(s, default)?)),
            Val::List(head) => {
                let head = parse_list_head(head, default)?;
                let tail = M::parse(Unit, default).unwrap();
                Some(ListMode::Form { head, tail })
            }
            Val::Pair(head_tail) => {
                let head_tail = Pair::from(head_tail);
                let Val::List(head) = head_tail.first else {
                    return None;
                };
                let head = parse_list_head(head, default)?;
                let tail = M::parse(head_tail.second, default)?;
                Some(ListMode::Form { head, tail })
            }
            _ => None,
        }
    }
}

fn parse_list_head<M: ParseMode<Val>>(head: ListVal, default: PrimitiveMode) -> Option<List<M>> {
    List::from(head)
        .into_iter()
        .map(|item| M::parse(item, default))
        .collect()
}

impl<M> GenerateMode<Val> for ListMode<M>
where
    M: GenerateMode<Val> + ParseMode<Unit> + PartialEq,
{
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            ListMode::Id => symbol(ID),
            ListMode::Form { head, tail } => {
                let head = generate_list_head(head, default);
                let tail_default = M::parse(Unit, default).unwrap() == *tail;
                if tail_default {
                    return head;
                }
                let tail = M::generate(tail, default);
                Val::Pair(Pair::new(head, tail).into())
            }
        }
    }
}

fn generate_list_head<M: GenerateMode<Val>>(head: &List<M>, default: PrimitiveMode) -> Val {
    let head: List<Val> = head.iter().map(|item| M::generate(item, default)).collect();
    Val::List(head.into())
}

impl<M> ParseMode<Val> for MapMode<M>
where
    M: ParseMode<Val> + ParseMode<Unit> + ParseMode<Symbol>,
    MapMode<M>: From<PrimitiveMode>,
{
    fn parse(mode: Val, default: PrimitiveMode) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(Self::from(default)),
            Val::Symbol(s) => Some(Self::from(PrimitiveMode::parse(s, default)?)),
            Val::Map(some) => {
                let some = parse_map_some(some, default)?;
                let default = M::parse(Unit, default).unwrap();
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

fn parse_map_some<M: ParseMode<Val>>(some: MapVal, default: PrimitiveMode) -> Option<Map<Val, M>> {
    Map::from(some)
        .into_iter()
        .map(|(k, v)| {
            let mode = M::parse(v, default)?;
            Some((k, mode))
        })
        .collect()
}

fn parse_map_else<M>(mode: Val, default: PrimitiveMode) -> Option<Pair<M, M>>
where
    M: ParseMode<Val> + ParseMode<Unit> + ParseMode<Symbol>,
{
    let mode = match mode {
        Val::Unit(_) => {
            let mode = M::parse(Unit, default).unwrap();
            Pair::new(mode.clone(), mode)
        }
        Val::Symbol(s) => {
            let mode = M::parse(s, default)?;
            Pair::new(mode.clone(), mode)
        }
        Val::Pair(else1) => {
            let else1 = Pair::from(else1);
            let key = M::parse(else1.first, default)?;
            let value = M::parse(else1.second, default)?;
            Pair::new(key, value)
        }
        _ => return None,
    };
    Some(mode)
}

impl<M> GenerateMode<Val> for MapMode<M>
where
    M: GenerateMode<Val> + ParseMode<Unit> + PartialEq,
{
    fn generate(&self, default: PrimitiveMode) -> Val {
        match self {
            MapMode::Id => symbol(ID),
            MapMode::Form { some, else1 } => {
                let some = generate_map_some(some, default);
                let default_mode = M::parse(Unit, default).unwrap();
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

fn generate_map_some<M: GenerateMode<Val>>(some: &Map<Val, M>, default: PrimitiveMode) -> Val {
    let some: Map<Val, Val> = some
        .iter()
        .map(|(k, v)| {
            let mode = M::generate(v, default);
            (k.clone(), mode)
        })
        .collect();
    Val::Map(some.into())
}

fn generate_map_else<M: GenerateMode<Val>>(k: &M, v: &M, default: PrimitiveMode) -> Val {
    let k = M::generate(k, default);
    let v = M::generate(v, default);
    Val::Pair(Pair::new(k, v).into())
}
