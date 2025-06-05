use crate::Bit;
use crate::Call;
use crate::List;
use crate::ListVal;
use crate::Map;
use crate::MapVal;
use crate::Mode;
use crate::Pair;
use crate::PairMode;
use crate::PrimMode;
use crate::Symbol;
use crate::SymbolMode;
use crate::Val;
use crate::mode::call::CallMode;
use crate::mode::comp::CompMode;
use crate::mode::list::ListMode;
use crate::mode::map::MapMode;
use crate::mode::prim::CodeMode;
use crate::mode::prim::DataMode;
use crate::mode::prim::EVAL;
use crate::mode::prim::EVAL_LITERAL;
use crate::mode::prim::EVAL_MOVE;
use crate::mode::prim::EVAL_REF;
use crate::mode::prim::FORM;
use crate::mode::prim::FORM_LITERAL;
use crate::mode::prim::FORM_MOVE;
use crate::mode::prim::FORM_REF;
use crate::mode::symbol::LITERAL;
use crate::mode::symbol::MOVE;
use crate::mode::symbol::REF;
use crate::utils::val::map_remove;
use crate::utils::val::symbol;
use crate::val::CALL;
use crate::val::LIST;
use crate::val::MAP;
use crate::val::PAIR;
use crate::val::SYMBOL;

pub(crate) trait ParseMode<T>: Sized + Clone {
    fn parse(mode: T) -> Option<Self>;
}

pub(crate) trait GenerateMode<T> {
    fn generate(&self) -> T;
}

pub(crate) fn parse(mode: Val) -> Option<Option<Mode>> {
    Option::<Mode>::parse(mode)
}

pub(crate) fn generate(mode: &Option<Mode>) -> Val {
    mode.generate()
}

impl<T: ParseMode<Val>> ParseMode<Val> for Option<T> {
    fn parse(mode: Val) -> Option<Self> {
        if mode.is_unit() {
            return Some(None);
        }
        Some(Some(T::parse(mode)?))
    }
}

impl<T: GenerateMode<Val>> GenerateMode<Val> for Option<T> {
    fn generate(&self) -> Val {
        match self {
            None => Val::default(),
            Some(mode) => mode.generate(),
        }
    }
}

const PRIMITIVE: &str = "primitive";

impl ParseMode<Val> for Mode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(symbol) => Some(Self::from(PrimMode::parse(symbol)?)),
            Val::Map(mut map) => {
                let primitive = match map_remove(&mut map, PRIMITIVE) {
                    Val::Unit(_) => false,
                    Val::Bit(b) => b.bool(),
                    _ => return None,
                };
                let mode = if primitive {
                    Mode::Prim(PrimMode::parse(map)?)
                } else {
                    Mode::Comp(Box::new(CompMode::parse(map)?))
                };
                Some(mode)
            }
            Val::Func(mode) => Some(Mode::Func(mode)),
            _ => None,
        }
    }
}

impl GenerateMode<Val> for Mode {
    fn generate(&self) -> Val {
        match self {
            Mode::Prim(mode) => mode.generate(),
            Mode::Comp(mode) => Val::Map(mode.generate()),
            Mode::Func(mode) => Val::Func(mode.clone()),
        }
    }
}

impl ParseMode<MapVal> for PrimMode {
    fn parse(mut map: MapVal) -> Option<Self> {
        let symbol = ParseMode::parse(map_remove(&mut map, SYMBOL))?;
        let pair = ParseMode::parse(map_remove(&mut map, PAIR))?;
        let call = ParseMode::parse(map_remove(&mut map, CALL))?;
        let list = ParseMode::parse(map_remove(&mut map, LIST))?;
        let map = ParseMode::parse(map_remove(&mut map, MAP))?;
        Some(PrimMode { symbol, pair, call, list, map })
    }
}

impl ParseMode<Symbol> for PrimMode {
    fn parse(mode: Symbol) -> Option<Self> {
        let mode = match &*mode {
            FORM_LITERAL => PrimMode::symbol_call(SymbolMode::Literal, CodeMode::Form),
            FORM_REF => PrimMode::symbol_call(SymbolMode::Ref, CodeMode::Form),
            FORM_MOVE => PrimMode::symbol_call(SymbolMode::Move, CodeMode::Form),
            EVAL_LITERAL => PrimMode::symbol_call(SymbolMode::Literal, CodeMode::Eval),
            EVAL_REF => PrimMode::symbol_call(SymbolMode::Ref, CodeMode::Eval),
            EVAL_MOVE => PrimMode::symbol_call(SymbolMode::Move, CodeMode::Eval),
            _ => return None,
        };
        Some(mode)
    }
}

impl GenerateMode<Val> for PrimMode {
    fn generate(&self) -> Val {
        if self.symbol.is_none()
            && self.pair.is_none()
            && self.call.is_none()
            && self.list.is_none()
            && self.map.is_none()
        {
            return Val::default();
        }

        if self.symbol.is_some()
            && self.pair.is_some()
            && self.call.is_some()
            && self.list.is_some()
            && self.map.is_some()
        {
            let s = match (self.call.unwrap(), self.symbol.unwrap()) {
                (CodeMode::Form, SymbolMode::Literal) => FORM_LITERAL,
                (CodeMode::Form, SymbolMode::Ref) => FORM_REF,
                (CodeMode::Form, SymbolMode::Move) => FORM_MOVE,
                (CodeMode::Eval, SymbolMode::Literal) => EVAL_LITERAL,
                (CodeMode::Eval, SymbolMode::Ref) => EVAL_REF,
                (CodeMode::Eval, SymbolMode::Move) => EVAL_MOVE,
            };
            return symbol(s);
        }

        Val::Map(self.generate())
    }
}

impl GenerateMode<MapVal> for PrimMode {
    fn generate(&self) -> MapVal {
        let mut map = Map::default();
        put_some(&mut map, SYMBOL, &self.symbol);
        put_some(&mut map, PAIR, &self.pair);
        put_some(&mut map, CALL, &self.call);
        put_some(&mut map, LIST, &self.list);
        put_some(&mut map, MAP, &self.map);
        map.insert(symbol(PRIMITIVE), Val::Bit(Bit::true1()));
        map.into()
    }
}

impl ParseMode<Val> for DataMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for DataMode {
    fn parse(mode: Symbol) -> Option<Self> {
        match &*mode {
            FORM => Some(DataMode),
            _ => None,
        }
    }
}

impl GenerateMode<Val> for DataMode {
    fn generate(&self) -> Val {
        Val::Symbol(self.generate())
    }
}

impl GenerateMode<Symbol> for DataMode {
    fn generate(&self) -> Symbol {
        Symbol::from_str(FORM)
    }
}

impl ParseMode<Val> for CodeMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for CodeMode {
    fn parse(mode: Symbol) -> Option<Self> {
        match &*mode {
            FORM => Some(CodeMode::Form),
            EVAL => Some(CodeMode::Eval),
            _ => None,
        }
    }
}

impl GenerateMode<Val> for CodeMode {
    fn generate(&self) -> Val {
        Val::Symbol(self.generate())
    }
}

impl GenerateMode<Symbol> for CodeMode {
    fn generate(&self) -> Symbol {
        let s = match self {
            CodeMode::Form => FORM,
            CodeMode::Eval => EVAL,
        };
        Symbol::from_str(s)
    }
}

impl ParseMode<Val> for SymbolMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s),
            _ => None,
        }
    }
}

impl ParseMode<Symbol> for SymbolMode {
    fn parse(mode: Symbol) -> Option<Self> {
        let mode = match &*mode {
            LITERAL => SymbolMode::Literal,
            REF => SymbolMode::Ref,
            MOVE => SymbolMode::Move,
            _ => return None,
        };
        Some(mode)
    }
}

impl GenerateMode<Val> for SymbolMode {
    fn generate(&self) -> Val {
        Val::Symbol(self.generate())
    }
}

impl GenerateMode<Symbol> for SymbolMode {
    fn generate(&self) -> Symbol {
        let s = match self {
            SymbolMode::Literal => LITERAL,
            SymbolMode::Ref => REF,
            SymbolMode::Move => MOVE,
        };
        Symbol::from_str(s)
    }
}

impl ParseMode<MapVal> for CompMode {
    fn parse(mut map: MapVal) -> Option<Self> {
        let symbol = ParseMode::parse(map_remove(&mut map, SYMBOL))?;
        let pair = ParseMode::parse(map_remove(&mut map, PAIR))?;
        let call = ParseMode::parse(map_remove(&mut map, CALL))?;
        let list = ParseMode::parse(map_remove(&mut map, LIST))?;
        let map = ParseMode::parse(map_remove(&mut map, MAP))?;
        Some(CompMode { symbol, pair, call, list, map })
    }
}

impl GenerateMode<MapVal> for CompMode {
    fn generate(&self) -> MapVal {
        let mut map = Map::default();
        put_some(&mut map, SYMBOL, &self.symbol);
        put_some(&mut map, PAIR, &self.pair);
        put_some(&mut map, CALL, &self.call);
        put_some(&mut map, LIST, &self.list);
        put_some(&mut map, MAP, &self.map);
        map.into()
    }
}

fn put_some<M>(map: &mut Map<Val, Val>, key: &'static str, mode: &Option<M>)
where M: GenerateMode<Val> {
    if let Some(mode) = mode {
        map.insert(symbol(key), mode.generate());
    }
}

impl ParseMode<Val> for PairMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(PrimMode::parse(s)?)),
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                let first = ParseMode::parse(pair.first)?;
                let second = ParseMode::parse(pair.second)?;
                Some(PairMode { first, second })
            }
            _ => None,
        }
    }
}

impl GenerateMode<Val> for PairMode {
    fn generate(&self) -> Val {
        let first = GenerateMode::generate(&self.first);
        let second = GenerateMode::generate(&self.second);
        Val::Pair(Pair::new(first, second).into())
    }
}

// todo design
impl ParseMode<Val> for CallMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::try_from(PrimMode::parse(s)?).unwrap()),
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    return None;
                };
                let some = parse_map_some(some)?;
                let else1 = parse_map_else(some_else.second)?;
                Some(CallMode { func: else1.first, input: else1.second, some: Some(some) })
            }
            Val::Call(call) => {
                let call = Call::from(call);
                let func = ParseMode::parse(call.func)?;
                let input = ParseMode::parse(call.input)?;
                Some(CallMode { some: None, func, input })
            }
            _ => None,
        }
    }
}

// todo design
impl GenerateMode<Val> for CallMode {
    fn generate(&self) -> Val {
        let func = GenerateMode::generate(&self.func);
        let input = GenerateMode::generate(&self.input);
        match &self.some {
            Some(some) => {
                let some = generate_map_some(some);
                let else1 = Val::Pair(Pair::new(func, input).into());
                Val::Pair(Pair::new(some, else1).into())
            }
            None => Val::Call(Call::new(false, func, input).into()),
        }
    }
}

impl ParseMode<Val> for ListMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(PrimMode::parse(s)?)),
            Val::List(head) => {
                let head = parse_list_head(head)?;
                let tail = None;
                Some(ListMode { head, tail })
            }
            Val::Pair(head_tail) => {
                let head_tail = Pair::from(head_tail);
                let Val::List(head) = head_tail.first else {
                    return None;
                };
                let head = parse_list_head(head)?;
                let tail = ParseMode::parse(head_tail.second)?;
                Some(ListMode { head, tail })
            }
            _ => None,
        }
    }
}

fn parse_list_head(head: ListVal) -> Option<List<Option<Mode>>> {
    List::from(head).into_iter().map(ParseMode::parse).collect()
}

impl GenerateMode<Val> for ListMode {
    fn generate(&self) -> Val {
        let head = generate_list_head(&self.head);
        if self.tail.is_none() {
            return head;
        }
        let tail = GenerateMode::generate(&self.tail);
        Val::Pair(Pair::new(head, tail).into())
    }
}

fn generate_list_head(head: &List<Option<Mode>>) -> Val {
    let head: List<Val> = head.iter().map(GenerateMode::generate).collect();
    Val::List(head.into())
}

impl ParseMode<Val> for MapMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::from(PrimMode::parse(s)?)),
            Val::Map(some) => {
                let some = parse_map_some(some)?;
                let else1 = Pair::new(None, None);
                Some(MapMode { some, else1 })
            }
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    return None;
                };
                let some = parse_map_some(some)?;
                let else1 = parse_map_else(some_else.second)?;
                Some(MapMode { some, else1 })
            }
            _ => None,
        }
    }
}

fn parse_map_some(some: MapVal) -> Option<Map<Val, Option<Mode>>> {
    Map::from(some)
        .into_iter()
        .map(|(k, v)| {
            let mode = ParseMode::parse(v)?;
            Some((k, mode))
        })
        .collect()
}

fn parse_map_else(mode: Val) -> Option<Pair<Option<Mode>, Option<Mode>>> {
    let mode = match mode {
        Val::Unit(_) => Pair::new(None, None),
        Val::Symbol(s) => {
            let mode = Some(Mode::from(PrimMode::parse(s)?));
            Pair::new(mode.clone(), mode)
        }
        Val::Pair(else1) => {
            let else1 = Pair::from(else1);
            let key = ParseMode::parse(else1.first)?;
            let value = ParseMode::parse(else1.second)?;
            Pair::new(key, value)
        }
        _ => return None,
    };
    Some(mode)
}

impl GenerateMode<Val> for MapMode {
    fn generate(&self) -> Val {
        let some = generate_map_some(&self.some);
        if self.else1.first.is_none() && self.else1.second.is_none() {
            return some;
        }
        let else1 = generate_map_else(&self.else1.first, &self.else1.second);
        Val::Pair(Pair::new(some, else1).into())
    }
}

fn generate_map_some<M: GenerateMode<Val>>(some: &Map<Val, M>) -> Val {
    let some: Map<Val, Val> = some
        .iter()
        .map(|(k, v)| {
            let mode = M::generate(v);
            (k.clone(), mode)
        })
        .collect();
    Val::Map(some.into())
}

fn generate_map_else<M: GenerateMode<Val>>(k: &M, v: &M) -> Val {
    let k = M::generate(k);
    let v = M::generate(v);
    Val::Pair(Pair::new(k, v).into())
}
