use const_format::concatcp;
use log::error;

use crate::cfg::lib::adapter::CallAdapter;
use crate::cfg::lib::adapter::CallPrimAdapter;
use crate::cfg::lib::adapter::CompAdapter;
use crate::cfg::lib::adapter::CoreAdapter;
use crate::cfg::lib::adapter::ListAdapter;
use crate::cfg::lib::adapter::MapAdapter;
use crate::cfg::lib::adapter::PairAdapter;
use crate::cfg::lib::adapter::PrimAdapter;
use crate::cfg::lib::adapter::SymbolAdapter;
use crate::cfg::utils::map_remove;
use crate::cfg::utils::symbol;
use crate::semantics::core::SYMBOL_EVAL_CHAR;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::val::CALL;
use crate::semantics::val::LIST;
use crate::semantics::val::ListVal;
use crate::semantics::val::MAP;
use crate::semantics::val::MapVal;
use crate::semantics::val::PAIR;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Unit;

pub(in crate::cfg) trait ParseAdapter<T>: Sized + Clone {
    fn parse(adapter: T) -> Option<Self>;
}

pub(in crate::cfg) trait GenerateAdapter<T> {
    fn generate(&self) -> T;
}

pub(in crate::cfg) fn parse(adapter: Val) -> Option<CoreAdapter> {
    CoreAdapter::parse(adapter)
}

// todo design
#[expect(dead_code)]
pub(in crate::cfg) fn generate(adapter: &CoreAdapter) -> Val {
    adapter.generate()
}

impl<T: ParseAdapter<Val>> ParseAdapter<Val> for Box<T> {
    fn parse(adapter: Val) -> Option<Self> {
        Some(Box::new(T::parse(adapter)?))
    }
}

impl<T: GenerateAdapter<Val>> GenerateAdapter<Val> for Box<T> {
    fn generate(&self) -> Val {
        (**self).generate()
    }
}

impl<T: ParseAdapter<Val>> ParseAdapter<Val> for Option<T> {
    fn parse(adapter: Val) -> Option<Self> {
        if adapter.is_unit() {
            return Some(None);
        }
        Some(Some(T::parse(adapter)?))
    }
}

impl<T: GenerateAdapter<Val>> GenerateAdapter<Val> for Option<T> {
    fn generate(&self) -> Val {
        match self {
            None => Val::default(),
            Some(adapter) => adapter.generate(),
        }
    }
}

impl ParseAdapter<Val> for CoreAdapter {
    fn parse(adapter: Val) -> Option<Self> {
        match adapter {
            Val::Unit(unit) => Self::parse(unit),
            Val::Symbol(symbol) => Self::parse(symbol),
            Val::Map(map) => Self::parse(map),
            Val::Func(func) => Some(CoreAdapter::Func(func)),
            _ => None,
        }
    }
}

impl ParseAdapter<Unit> for CoreAdapter {
    fn parse(_: Unit) -> Option<Self> {
        Some(Self::id())
    }
}

impl ParseAdapter<Symbol> for CoreAdapter {
    fn parse(adapter: Symbol) -> Option<Self> {
        Some(Self::from(PrimAdapter::parse(adapter)?))
    }
}

impl ParseAdapter<MapVal> for CoreAdapter {
    fn parse(adapter: MapVal) -> Option<Self> {
        Some(CoreAdapter::Comp(CompAdapter::parse(adapter)?))
    }
}

impl GenerateAdapter<Val> for CoreAdapter {
    fn generate(&self) -> Val {
        match self {
            CoreAdapter::Comp(adapter) => Val::Map(adapter.generate()),
            CoreAdapter::Func(adapter) => Val::Func(adapter.clone()),
        }
    }
}

impl ParseAdapter<Val> for PrimAdapter {
    fn parse(adapter: Val) -> Option<Self> {
        match adapter {
            Val::Unit(_) => Some(PrimAdapter::id()),
            Val::Symbol(s) => Self::parse(s),
            v => {
                error!("{v:?} should be a symbol");
                None
            }
        }
    }
}

// todo rename
pub(in crate::cfg) const FORM: &str = "form";
pub(in crate::cfg) const EVAL: &str = "eval";
pub(in crate::cfg) const SYMBOL_ID: &str = "";
pub(in crate::cfg) const FORM_ID: &str = concatcp!(FORM, SYMBOL_ID);
pub(in crate::cfg) const FORM_LITERAL: &str = concatcp!(FORM, SYMBOL_LITERAL_CHAR);
pub(in crate::cfg) const FORM_REF: &str = concatcp!(FORM, SYMBOL_REF_CHAR);
pub(in crate::cfg) const FORM_EVAL: &str = concatcp!(FORM, SYMBOL_EVAL_CHAR);
pub(in crate::cfg) const EVAL_ID: &str = concatcp!(EVAL, SYMBOL_ID);
pub(in crate::cfg) const EVAL_LITERAL: &str = concatcp!(EVAL, SYMBOL_LITERAL_CHAR);
pub(in crate::cfg) const EVAL_REF: &str = concatcp!(EVAL, SYMBOL_REF_CHAR);
pub(in crate::cfg) const EVAL_EVAL: &str = concatcp!(EVAL, SYMBOL_EVAL_CHAR);

impl ParseAdapter<Symbol> for PrimAdapter {
    fn parse(adapter: Symbol) -> Option<Self> {
        let adapter = match &*adapter {
            FORM_ID => PrimAdapter::new(SymbolAdapter::Id, CallPrimAdapter::Form),
            FORM_LITERAL => PrimAdapter::new(SymbolAdapter::Literal, CallPrimAdapter::Form),
            FORM_REF => PrimAdapter::new(SymbolAdapter::Ref, CallPrimAdapter::Form),
            FORM_EVAL => PrimAdapter::new(SymbolAdapter::Eval, CallPrimAdapter::Form),
            EVAL_ID => PrimAdapter::new(SymbolAdapter::Id, CallPrimAdapter::Eval),
            EVAL_LITERAL => PrimAdapter::new(SymbolAdapter::Literal, CallPrimAdapter::Eval),
            EVAL_REF => PrimAdapter::new(SymbolAdapter::Ref, CallPrimAdapter::Eval),
            EVAL_EVAL => PrimAdapter::new(SymbolAdapter::Eval, CallPrimAdapter::Eval),
            s => {
                error!("{s} should be a symbol representing a primitive adapter");
                return None;
            }
        };
        Some(adapter)
    }
}

impl GenerateAdapter<Val> for PrimAdapter {
    fn generate(&self) -> Val {
        if self.is_id() {
            return Val::default();
        }
        let s = match (self.call, self.symbol) {
            (CallPrimAdapter::Form, SymbolAdapter::Id) => FORM_ID,
            (CallPrimAdapter::Form, SymbolAdapter::Literal) => FORM_LITERAL,
            (CallPrimAdapter::Form, SymbolAdapter::Ref) => FORM_REF,
            (CallPrimAdapter::Form, SymbolAdapter::Eval) => FORM_EVAL,
            (CallPrimAdapter::Eval, SymbolAdapter::Id) => EVAL_ID,
            (CallPrimAdapter::Eval, SymbolAdapter::Literal) => EVAL_LITERAL,
            (CallPrimAdapter::Eval, SymbolAdapter::Ref) => EVAL_REF,
            (CallPrimAdapter::Eval, SymbolAdapter::Eval) => EVAL_EVAL,
        };
        symbol(s)
    }
}

const DEFAULT: &str = "default";

impl ParseAdapter<MapVal> for CompAdapter {
    fn parse(mut map: MapVal) -> Option<Self> {
        let default = ParseAdapter::parse(map_remove(&mut map, DEFAULT))?;
        let pair = ParseAdapter::parse(map_remove(&mut map, PAIR))?;
        let call = ParseAdapter::parse(map_remove(&mut map, CALL))?;
        let list = ParseAdapter::parse(map_remove(&mut map, LIST))?;
        let map = ParseAdapter::parse(map_remove(&mut map, MAP))?;
        Some(CompAdapter { default, pair, call, list, map })
    }
}

impl GenerateAdapter<MapVal> for CompAdapter {
    fn generate(&self) -> MapVal {
        let mut map = Map::default();
        let default = self.default.generate();
        if !default.is_unit() {
            map.insert(symbol(DEFAULT), default);
        }
        put_some(&mut map, PAIR, &self.pair);
        put_some(&mut map, CALL, &self.call);
        put_some(&mut map, LIST, &self.list);
        put_some(&mut map, MAP, &self.map);
        map.into()
    }
}

fn put_some<M>(map: &mut Map<Val, Val>, key: &'static str, adapter: &Option<M>)
where M: GenerateAdapter<Val> {
    if let Some(adapter) = adapter {
        map.insert(symbol(key), adapter.generate());
    }
}

impl ParseAdapter<Val> for PairAdapter {
    fn parse(adapter: Val) -> Option<Self> {
        match adapter {
            Val::Symbol(symbol) => {
                let adapter = CoreAdapter::parse(symbol)?;
                Some(PairAdapter { some: Map::default(), first: adapter.clone(), second: adapter })
            }
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    error!("first {:?} should be a map", some_else.first);
                    return None;
                };
                let some = parse_map_some(some)?;
                let Val::Pair(pair) = some_else.second else {
                    error!("second {:?} should be a pair", some_else.second);
                    return None;
                };
                let pair = Pair::from(pair);
                let first = ParseAdapter::parse(pair.first)?;
                let second = ParseAdapter::parse(pair.second)?;
                Some(PairAdapter { some, first, second })
            }
            v => {
                error!("{v:?} should be a pair or a symbol");
                None
            }
        }
    }
}

impl GenerateAdapter<Val> for PairAdapter {
    fn generate(&self) -> Val {
        let some = generate_map_some(&self.some);
        let first = GenerateAdapter::generate(&self.first);
        let second = GenerateAdapter::generate(&self.second);
        let else_ = Val::Pair(Pair::new(first, second).into());
        Val::Pair(Pair::new(some, else_).into())
    }
}

impl ParseAdapter<Val> for CallAdapter {
    fn parse(adapter: Val) -> Option<Self> {
        match adapter {
            Val::Symbol(symbol) => {
                let adapter = CoreAdapter::parse(symbol)?;
                Some(CallAdapter { func: adapter.clone(), input: adapter })
            }
            Val::Call(call) => {
                let call = Call::from(call);
                let func = ParseAdapter::parse(call.func)?;
                let input = ParseAdapter::parse(call.input)?;
                Some(CallAdapter { func, input })
            }
            v => {
                error!("{v:?} should be a call, a pair or a symbol");
                None
            }
        }
    }
}

impl GenerateAdapter<Val> for CallAdapter {
    fn generate(&self) -> Val {
        let func = GenerateAdapter::generate(&self.func);
        let input = GenerateAdapter::generate(&self.input);
        Val::Call(Call { func, input }.into())
    }
}

impl ParseAdapter<Val> for ListAdapter {
    fn parse(adapter: Val) -> Option<Self> {
        match adapter {
            Val::Symbol(symbol) => {
                let adapter = CoreAdapter::parse(symbol)?;
                Some(ListAdapter { head: List::default(), tail: adapter })
            }
            Val::List(head) => {
                let head = parse_list_head(head)?;
                let tail = CoreAdapter::id();
                Some(ListAdapter { head, tail })
            }
            Val::Pair(head_tail) => {
                let head_tail = Pair::from(head_tail);
                let Val::List(head) = head_tail.first else {
                    error!("first {:?} should be a list", head_tail.first);
                    return None;
                };
                let head = parse_list_head(head)?;
                let tail = ParseAdapter::parse(head_tail.second)?;
                Some(ListAdapter { head, tail })
            }
            v => {
                error!("{v:?} should be a list, a pair or a symbol");
                None
            }
        }
    }
}

fn parse_list_head(head: ListVal) -> Option<List<CoreAdapter>> {
    List::from(head).into_iter().map(ParseAdapter::parse).collect()
}

impl GenerateAdapter<Val> for ListAdapter {
    fn generate(&self) -> Val {
        let head = generate_list_head(&self.head);
        if self.tail.is_id() {
            return head;
        }
        let tail = GenerateAdapter::generate(&self.tail);
        Val::Pair(Pair::new(head, tail).into())
    }
}

fn generate_list_head(head: &List<CoreAdapter>) -> Val {
    let head: List<Val> = head.iter().map(GenerateAdapter::generate).collect();
    Val::List(head.into())
}

impl ParseAdapter<Val> for MapAdapter {
    fn parse(adapter: Val) -> Option<Self> {
        match adapter {
            Val::Symbol(symbol) => {
                let adapter = CoreAdapter::parse(symbol)?;
                Some(MapAdapter { some: Map::default(), else_: adapter })
            }
            Val::Map(some) => {
                let some = parse_map_some(some)?;
                let else_ = CoreAdapter::id();
                Some(MapAdapter { some, else_ })
            }
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    error!("first {:?} should be a map", some_else.first);
                    return None;
                };
                let some = parse_map_some(some)?;
                let else_ = ParseAdapter::parse(some_else.second)?;
                Some(MapAdapter { some, else_ })
            }
            _ => None,
        }
    }
}

fn parse_map_some(some: MapVal) -> Option<Map<Val, CoreAdapter>> {
    Map::from(some)
        .into_iter()
        .map(|(k, v)| {
            let adapter = ParseAdapter::parse(v)?;
            Some((k, adapter))
        })
        .collect()
}

impl GenerateAdapter<Val> for MapAdapter {
    fn generate(&self) -> Val {
        let some = generate_map_some(&self.some);
        if self.else_.is_id() {
            return some;
        }
        let else_ = self.else_.generate();
        Val::Pair(Pair::new(some, else_).into())
    }
}

fn generate_map_some<M: GenerateAdapter<Val>>(some: &Map<Val, M>) -> Val {
    let some: Map<Val, Val> = some
        .iter()
        .map(|(k, v)| {
            let adapter = M::generate(v);
            (k.clone(), adapter)
        })
        .collect();
    Val::Map(some.into())
}
