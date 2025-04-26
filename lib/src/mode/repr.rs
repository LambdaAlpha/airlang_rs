use crate::{
    Bit,
    Call,
    EitherMode,
    List,
    ListVal,
    Map,
    MapVal,
    Mode,
    Pair,
    PairMode,
    PrimMode,
    Symbol,
    SymbolMode,
    UniMode,
    Val,
    mode::{
        call::CallMode,
        change::ChangeMode,
        comp::CompMode,
        id::ID,
        list::ListMode,
        map::MapMode,
        prim::{
            CodeMode,
            DataMode,
        },
        symbol::{
            LITERAL,
            MOVE,
            REF,
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
        },
    },
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        CALL,
        CHANGE,
        EITHER,
        LIST,
        MAP,
        PAIR,
        SYMBOL,
    },
};

pub(crate) trait ParseMode<T>: Sized + Clone {
    fn parse(mode: T, default: Option<UniMode>) -> Option<Self>;
}

pub(crate) trait GenerateMode<T> {
    fn generate(&self, default: Option<UniMode>) -> T;
}

pub(crate) fn parse(mode: Val) -> Option<Option<Mode>> {
    Option::<Mode>::parse(mode, None)
}

pub(crate) fn generate(mode: &Option<Mode>) -> Val {
    mode.generate(None)
}

impl<T> ParseMode<Val> for Option<T>
where T: ParseMode<Val> + From<UniMode>
{
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        let mode = match mode {
            Val::Unit(_) => default.map(Into::into),
            Val::Symbol(s) => match &*s {
                ID => None,
                _ => Some(T::parse(Val::Symbol(s), default)?),
            },
            v => Some(T::parse(v, default)?),
        };
        Some(mode)
    }
}

impl<T: ParseMode<Symbol>> ParseMode<Symbol> for Option<T> {
    fn parse(mode: Symbol, default: Option<UniMode>) -> Option<Self> {
        let mode = match &*mode {
            ID => None,
            _ => Some(T::parse(mode, default)?),
        };
        Some(mode)
    }
}

impl<T: GenerateMode<Val>> GenerateMode<Val> for Option<T> {
    fn generate(&self, default: Option<UniMode>) -> Val {
        match self {
            None => symbol(ID),
            Some(mode) => mode.generate(default),
        }
    }
}

const DEFAULT: &str = "default";
const PRIMITIVE: &str = "primitive";

impl ParseMode<Val> for Mode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
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
            Val::Func(mode) => Some(Mode::Func(mode)),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for Mode {
    fn parse(mode: Symbol, default: Option<UniMode>) -> Option<Self> {
        let mode = UniMode::parse(mode, default)?;
        Some(Mode::from(mode))
    }
}

impl GenerateMode<Val> for Mode {
    fn generate(&self, default: Option<UniMode>) -> Val {
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
            Mode::Func(mode) => Val::Func(mode.clone()),
        }
    }
}

impl ParseMode<Val> for UniMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for UniMode {
    fn parse(mode: Symbol, _default: Option<UniMode>) -> Option<Self> {
        let mode = match &*mode {
            FORM_LITERAL => UniMode::new(CodeMode::Form, SymbolMode::Literal),
            FORM_REF => UniMode::new(CodeMode::Form, SymbolMode::Ref),
            FORM_MOVE => UniMode::new(CodeMode::Form, SymbolMode::Move),
            EVAL_LITERAL => UniMode::new(CodeMode::Eval, SymbolMode::Literal),
            EVAL_REF => UniMode::new(CodeMode::Eval, SymbolMode::Ref),
            EVAL_MOVE => UniMode::new(CodeMode::Eval, SymbolMode::Move),
            _ => return None,
        };
        Some(mode)
    }
}

impl GenerateMode<Symbol> for UniMode {
    fn generate(&self, _default: Option<UniMode>) -> Symbol {
        let s = match self.code {
            CodeMode::Form => match self.symbol {
                SymbolMode::Literal => FORM_LITERAL,
                SymbolMode::Ref => FORM_REF,
                SymbolMode::Move => FORM_MOVE,
            },
            CodeMode::Eval => match self.symbol {
                SymbolMode::Literal => EVAL_LITERAL,
                SymbolMode::Ref => EVAL_REF,
                SymbolMode::Move => EVAL_MOVE,
            },
        };
        Symbol::from_str(s)
    }
}

impl ParseMode<Val> for DataMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for DataMode {
    fn parse(mode: Symbol, _default: Option<UniMode>) -> Option<Self> {
        match &*mode {
            FORM => Some(DataMode),
            _ => None,
        }
    }
}

impl GenerateMode<Symbol> for DataMode {
    fn generate(&self, _default: Option<UniMode>) -> Symbol {
        Symbol::from_str(FORM)
    }
}

impl GenerateMode<Val> for DataMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        Val::Symbol(self.generate(default))
    }
}

impl ParseMode<Val> for CodeMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for CodeMode {
    fn parse(mode: Symbol, _default: Option<UniMode>) -> Option<Self> {
        match &*mode {
            FORM => Some(CodeMode::Form),
            EVAL => Some(CodeMode::Eval),
            _ => None,
        }
    }
}

impl GenerateMode<Symbol> for CodeMode {
    fn generate(&self, _default: Option<UniMode>) -> Symbol {
        let s = match self {
            CodeMode::Form => FORM,
            CodeMode::Eval => EVAL,
        };
        Symbol::from_str(s)
    }
}

impl GenerateMode<Val> for CodeMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        Val::Symbol(self.generate(default))
    }
}

impl ParseMode<MapVal> for CompMode {
    fn parse(mut map: MapVal, default: Option<UniMode>) -> Option<Self> {
        let default = ParseMode::parse(map_remove(&mut map, DEFAULT), default)?;
        let symbol = ParseMode::parse(map_remove(&mut map, SYMBOL), default)?;
        let pair = ParseMode::parse(map_remove(&mut map, PAIR), default)?;
        let either = ParseMode::parse(map_remove(&mut map, EITHER), default)?;
        let change = ParseMode::parse(map_remove(&mut map, CHANGE), default)?;
        let call = ParseMode::parse(map_remove(&mut map, CALL), default)?;
        let list = ParseMode::parse(map_remove(&mut map, LIST), default)?;
        let map = ParseMode::parse(map_remove(&mut map, MAP), default)?;
        Some(CompMode { symbol, pair, either, change, call, list, map })
    }
}

impl GenerateMode<MapVal> for CompMode {
    fn generate(&self, default: Option<UniMode>) -> MapVal {
        let mut map = Map::default();
        put_non_default(&mut map, default, &self.symbol, SYMBOL);
        put_non_default(&mut map, default, &self.pair, PAIR);
        put_non_default(&mut map, default, &self.either, EITHER);
        put_non_default(&mut map, default, &self.change, CHANGE);
        put_non_default(&mut map, default, &self.call, CALL);
        put_non_default(&mut map, default, &self.list, LIST);
        put_non_default(&mut map, default, &self.map, MAP);
        map.into()
    }
}

impl ParseMode<MapVal> for PrimMode {
    fn parse(mut map: MapVal, default: Option<UniMode>) -> Option<Self> {
        let default = ParseMode::parse(map_remove(&mut map, DEFAULT), default)?;
        let symbol = ParseMode::parse(map_remove(&mut map, SYMBOL), default)?;
        let pair = ParseMode::parse(map_remove(&mut map, PAIR), default)?;
        let either = ParseMode::parse(map_remove(&mut map, EITHER), default)?;
        let change = ParseMode::parse(map_remove(&mut map, CHANGE), default)?;
        let call = ParseMode::parse(map_remove(&mut map, CALL), default)?;
        let list = ParseMode::parse(map_remove(&mut map, LIST), default)?;
        let map = ParseMode::parse(map_remove(&mut map, MAP), default)?;
        Some(PrimMode { symbol, pair, either, change, call, list, map })
    }
}

impl GenerateMode<MapVal> for PrimMode {
    fn generate(&self, default: Option<UniMode>) -> MapVal {
        let mut map = Map::default();
        put_non_default(&mut map, default, &self.symbol, SYMBOL);
        put_non_default(&mut map, default, &self.pair, PAIR);
        put_non_default(&mut map, default, &self.either, EITHER);
        put_non_default(&mut map, default, &self.change, CHANGE);
        put_non_default(&mut map, default, &self.call, CALL);
        put_non_default(&mut map, default, &self.list, LIST);
        put_non_default(&mut map, default, &self.map, MAP);
        map.into()
    }
}

fn put_non_default<M>(
    map: &mut Map<Val, Val>, default: Option<UniMode>, mode: &Option<M>, key: &'static str,
) where M: GenerateMode<Val> + Eq + From<UniMode> {
    if default.map(Into::into) != *mode {
        map.insert(symbol(key), mode.generate(default));
    }
}

impl ParseMode<Val> for SymbolMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s, default),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for SymbolMode {
    fn parse(mode: Symbol, _default: Option<UniMode>) -> Option<Self> {
        let mode = match &*mode {
            LITERAL => SymbolMode::Literal,
            REF => SymbolMode::Ref,
            MOVE => SymbolMode::Move,
            _ => return None,
        };
        Some(mode)
    }
}

impl GenerateMode<Symbol> for SymbolMode {
    fn generate(&self, _default: Option<UniMode>) -> Symbol {
        let s = match self {
            SymbolMode::Literal => LITERAL,
            SymbolMode::Ref => REF,
            SymbolMode::Move => MOVE,
        };
        Symbol::from_str(s)
    }
}

impl GenerateMode<Val> for SymbolMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        Val::Symbol(self.generate(default))
    }
}

impl ParseMode<Val> for PairMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let first = ParseMode::parse(pair.first, default)?;
                let second = ParseMode::parse(pair.second, default)?;
                Some(PairMode { first, second })
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for PairMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        let first = GenerateMode::generate(&self.first, default);
        let second = GenerateMode::generate(&self.second, default);
        Val::Pair(Pair::new(first, second).into())
    }
}

impl ParseMode<Val> for EitherMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let this = ParseMode::parse(pair.first, default)?;
                let that = ParseMode::parse(pair.second, default)?;
                Some(EitherMode { this, that })
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for EitherMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        let this = GenerateMode::generate(&self.this, default);
        let that = GenerateMode::generate(&self.that, default);
        Val::Pair(Pair::new(this, that).into())
    }
}

impl ParseMode<Val> for ChangeMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let from = ParseMode::parse(pair.first, default)?;
                let to = ParseMode::parse(pair.second, default)?;
                Some(ChangeMode { from, to })
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for ChangeMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        let from = GenerateMode::generate(&self.from, default);
        let to = GenerateMode::generate(&self.to, default);
        Val::Pair(Pair::new(from, to).into())
    }
}

impl ParseMode<Val> for CallMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let func = ParseMode::parse(pair.first, default)?;
                let input = ParseMode::parse(pair.second, default)?;
                Some(CallMode { code: CodeMode::Form, func, input })
            }
            Val::Call(call) => {
                let call = Call::from(call);
                let func = ParseMode::parse(call.func, default)?;
                let input = ParseMode::parse(call.input, default)?;
                Some(CallMode { code: CodeMode::Eval, func, input })
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for CallMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        let func = GenerateMode::generate(&self.func, default);
        let input = GenerateMode::generate(&self.input, default);
        match self.code {
            CodeMode::Form => Val::Pair(Pair::new(func, input).into()),
            CodeMode::Eval => Val::Call(Call::new(func, input).into()),
        }
    }
}

impl ParseMode<Val> for ListMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::List(head) => {
                let head = parse_list_head(head, default)?;
                let tail = default.map(Into::into);
                Some(ListMode { head, tail })
            }
            Val::Pair(head_tail) => {
                let head_tail = Pair::from(head_tail);
                let Val::List(head) = head_tail.first else {
                    return None;
                };
                let head = parse_list_head(head, default)?;
                let tail = ParseMode::parse(head_tail.second, default)?;
                Some(ListMode { head, tail })
            }
            _ => None,
        }
    }
}

fn parse_list_head(head: ListVal, default: Option<UniMode>) -> Option<List<Option<Mode>>> {
    List::from(head).into_iter().map(|item| ParseMode::parse(item, default)).collect()
}

impl GenerateMode<Val> for ListMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        let head = generate_list_head(&self.head, default);
        let tail_default = default.map(Into::into) == self.tail;
        if tail_default {
            return head;
        }
        let tail = GenerateMode::generate(&self.tail, default);
        Val::Pair(Pair::new(head, tail).into())
    }
}

fn generate_list_head(head: &List<Option<Mode>>, default: Option<UniMode>) -> Val {
    let head: List<Val> = head.iter().map(|item| GenerateMode::generate(item, default)).collect();
    Val::List(head.into())
}

impl ParseMode<Val> for MapMode {
    fn parse(mode: Val, default: Option<UniMode>) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(UniMode::parse(s, default)?)),
            Val::Map(some) => {
                let some = parse_map_some(some, default)?;
                let default = default.map(Into::into);
                let else1 = Pair::new(default.clone(), default);
                Some(MapMode { some, else1 })
            }
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    return None;
                };
                let some = parse_map_some(some, default)?;
                let else1 = parse_map_else(some_else.second, default)?;
                Some(MapMode { some, else1 })
            }
            _ => None,
        }
    }
}

fn parse_map_some(some: MapVal, default: Option<UniMode>) -> Option<Map<Val, Option<Mode>>> {
    Map::from(some)
        .into_iter()
        .map(|(k, v)| {
            let mode = ParseMode::parse(v, default)?;
            Some((k, mode))
        })
        .collect()
}

fn parse_map_else(mode: Val, default: Option<UniMode>) -> Option<Pair<Option<Mode>, Option<Mode>>> {
    let mode = match mode {
        Val::Unit(_) => {
            let mode = default.map(Into::into);
            Pair::new(mode.clone(), mode)
        }
        Val::Symbol(s) => {
            let mode = Option::<Mode>::parse(s, default)?;
            Pair::new(mode.clone(), mode)
        }
        Val::Pair(else1) => {
            let else1 = Pair::from(else1);
            let key = ParseMode::parse(else1.first, default)?;
            let value = ParseMode::parse(else1.second, default)?;
            Pair::new(key, value)
        }
        _ => return None,
    };
    Some(mode)
}

impl GenerateMode<Val> for MapMode {
    fn generate(&self, default: Option<UniMode>) -> Val {
        let some = generate_map_some(&self.some, default);
        let default_mode = default.map(Into::into);
        let else_default = default_mode == self.else1.first && default_mode == self.else1.second;
        if else_default {
            return some;
        }
        let else1 = generate_map_else(&self.else1.first, &self.else1.second, default);
        Val::Pair(Pair::new(some, else1).into())
    }
}

fn generate_map_some<M: GenerateMode<Val>>(some: &Map<Val, M>, default: Option<UniMode>) -> Val {
    let some: Map<Val, Val> = some
        .iter()
        .map(|(k, v)| {
            let mode = M::generate(v, default);
            (k.clone(), mode)
        })
        .collect();
    Val::Map(some.into())
}

fn generate_map_else<M: GenerateMode<Val>>(k: &M, v: &M, default: Option<UniMode>) -> Val {
    let k = M::generate(k, default);
    let v = M::generate(v, default);
    Val::Pair(Pair::new(k, v).into())
}
