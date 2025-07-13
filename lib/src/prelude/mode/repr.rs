use const_format::concatcp;
use log::error;

use super::CallMapMode;
use super::CallMode;
use super::CodeMode;
use super::CompMode;
use super::DataMode;
use super::ListMode;
use super::MapMode;
use super::Mode;
use super::PairMode;
use super::PrimMode;
use super::SymbolMode;
use crate::prelude::utils::map_remove;
use crate::prelude::utils::symbol;
use crate::semantics::core::SYMBOL_EVAL;
use crate::semantics::core::SYMBOL_EVAL_CHAR;
use crate::semantics::core::SYMBOL_LITERAL;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_MOVE;
use crate::semantics::core::SYMBOL_MOVE_CHAR;
use crate::semantics::core::SYMBOL_REF;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::val::CALL;
use crate::semantics::val::LIST;
use crate::semantics::val::ListVal;
use crate::semantics::val::MAP;
use crate::semantics::val::MapVal;
use crate::semantics::val::PAIR;
use crate::semantics::val::SYMBOL;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Call;
use crate::type_::CtxInput;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;

pub(in crate::prelude) trait ParseMode<T>: Sized + Clone {
    fn parse(mode: T) -> Option<Self>;
}

pub(in crate::prelude) trait GenerateMode<T> {
    fn generate(&self) -> T;
}

pub(in crate::prelude) fn parse(mode: Val) -> Option<Option<Mode>> {
    Option::<Mode>::parse(mode)
}

// todo design
#[expect(dead_code)]
pub(in crate::prelude) fn generate(mode: &Option<Mode>) -> Val {
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
                    v => {
                        error!("primitive {v:?} should be a bit or a unit");
                        return None;
                    }
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

// todo rename
pub(in crate::prelude) const FORM: &str = "form";
pub(in crate::prelude) const EVAL: &str = "eval";
pub(in crate::prelude) const FORM_LITERAL: &str = concatcp!(FORM, SYMBOL_LITERAL_CHAR);
pub(in crate::prelude) const FORM_REF: &str = concatcp!(FORM, SYMBOL_REF_CHAR);
pub(in crate::prelude) const FORM_MOVE: &str = concatcp!(FORM, SYMBOL_MOVE_CHAR);
pub(in crate::prelude) const FORM_EVAL: &str = concatcp!(FORM, SYMBOL_EVAL_CHAR);
pub(in crate::prelude) const EVAL_LITERAL: &str = concatcp!(EVAL, SYMBOL_LITERAL_CHAR);
pub(in crate::prelude) const EVAL_REF: &str = concatcp!(EVAL, SYMBOL_REF_CHAR);
pub(in crate::prelude) const EVAL_MOVE: &str = concatcp!(EVAL, SYMBOL_MOVE_CHAR);
pub(in crate::prelude) const EVAL_EVAL: &str = concatcp!(EVAL, SYMBOL_EVAL_CHAR);

impl ParseMode<Symbol> for PrimMode {
    fn parse(mode: Symbol) -> Option<Self> {
        let mode = match &*mode {
            FORM_LITERAL => PrimMode::symbol_call(SymbolMode::Literal, CodeMode::Form),
            FORM_REF => PrimMode::symbol_call(SymbolMode::Ref, CodeMode::Form),
            FORM_MOVE => PrimMode::symbol_call(SymbolMode::Move, CodeMode::Form),
            FORM_EVAL => PrimMode::symbol_call(SymbolMode::Eval, CodeMode::Form),
            EVAL_LITERAL => PrimMode::symbol_call(SymbolMode::Literal, CodeMode::Eval),
            EVAL_REF => PrimMode::symbol_call(SymbolMode::Ref, CodeMode::Eval),
            EVAL_MOVE => PrimMode::symbol_call(SymbolMode::Move, CodeMode::Eval),
            EVAL_EVAL => PrimMode::symbol_call(SymbolMode::Eval, CodeMode::Eval),
            s => {
                error!("{s} should be a symbol representing a primitive mode");
                return None;
            }
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
                (CodeMode::Form, SymbolMode::Eval) => FORM_EVAL,
                (CodeMode::Eval, SymbolMode::Literal) => EVAL_LITERAL,
                (CodeMode::Eval, SymbolMode::Ref) => EVAL_REF,
                (CodeMode::Eval, SymbolMode::Move) => EVAL_MOVE,
                (CodeMode::Eval, SymbolMode::Eval) => EVAL_EVAL,
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
        map.insert(symbol(PRIMITIVE), Val::Bit(Bit::true_()));
        map.into()
    }
}

impl ParseMode<Val> for DataMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s),
            v => {
                error!("{v:?} should be a symbol");
                None
            }
        }
    }
}

impl ParseMode<Symbol> for DataMode {
    fn parse(mode: Symbol) -> Option<Self> {
        match &*mode {
            FORM => Some(DataMode),
            s => {
                error!("{s} should be a symbol representing a data mode");
                None
            }
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
        Symbol::from_str_unchecked(FORM)
    }
}

impl ParseMode<Val> for CodeMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s),
            v => {
                error!("{v:?} should be a symbol");
                None
            }
        }
    }
}

impl ParseMode<Symbol> for CodeMode {
    fn parse(mode: Symbol) -> Option<Self> {
        match &*mode {
            FORM => Some(CodeMode::Form),
            EVAL => Some(CodeMode::Eval),
            s => {
                error!("{s} should be a symbol representing a code mode");
                None
            }
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
        Symbol::from_str_unchecked(s)
    }
}

impl ParseMode<Val> for SymbolMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Self::parse(s),
            v => {
                error!("{v:?} should be a symbol");
                None
            }
        }
    }
}

impl ParseMode<Symbol> for SymbolMode {
    fn parse(mode: Symbol) -> Option<Self> {
        let mode = match &*mode {
            SYMBOL_LITERAL => SymbolMode::Literal,
            SYMBOL_REF => SymbolMode::Ref,
            SYMBOL_MOVE => SymbolMode::Move,
            SYMBOL_EVAL => SymbolMode::Eval,
            s => {
                error!("{s} should be a symbol representing a symbol mode");
                return None;
            }
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
            SymbolMode::Literal => SYMBOL_LITERAL,
            SymbolMode::Ref => SYMBOL_REF,
            SymbolMode::Move => SYMBOL_MOVE,
            SymbolMode::Eval => SYMBOL_EVAL,
        };
        Symbol::from_str_unchecked(s)
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
            v => {
                error!("{v:?} should be a pair or a symbol");
                None
            }
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
                    error!("first {:?} should be a map", some_else.first);
                    return None;
                };
                let some = parse_call_map_some(some)?;
                let Val::Call(call) = some_else.second else {
                    error!("second {:?} should be a call", some_else.second);
                    return None;
                };
                let call = Call::from(call);
                let func = ParseMode::parse(call.func)?;
                let ctx = ParseMode::parse(call.ctx)?;
                let input = ParseMode::parse(call.input)?;
                Some(CallMode { func, ctx, input, some: Some(some) })
            }
            Val::Call(call) => {
                let call = Call::from(call);
                let func = ParseMode::parse(call.func)?;
                let ctx = ParseMode::parse(call.ctx)?;
                let input = ParseMode::parse(call.input)?;
                Some(CallMode { some: None, func, ctx, input })
            }
            v => {
                error!("{v:?} should be a call, a pair or a symbol");
                None
            }
        }
    }
}

fn parse_call_map_some(some: MapVal) -> Option<CallMapMode> {
    Map::from(some)
        .into_iter()
        .map(|(k, v)| {
            let Val::Pair(ctx_input) = v else {
                return None;
            };
            let ctx_input = Pair::from(ctx_input);
            let ctx = ParseMode::parse(ctx_input.first)?;
            let input = ParseMode::parse(ctx_input.second)?;
            Some((k, CtxInput { ctx, input }))
        })
        .collect()
}

// todo design
impl GenerateMode<Val> for CallMode {
    fn generate(&self) -> Val {
        let func = GenerateMode::generate(&self.func);
        let ctx = GenerateMode::generate(&self.ctx);
        let input = GenerateMode::generate(&self.input);
        match &self.some {
            Some(some) => {
                let some = generate_call_map_some(some);
                let else_ = Val::Call(Call::new(false, func, ctx, input).into());
                Val::Pair(Pair::new(some, else_).into())
            }
            None => Val::Call(Call::new(false, func, ctx, input).into()),
        }
    }
}

fn generate_call_map_some(some: &Map<Val, CtxInput<Option<Mode>, Option<Mode>>>) -> Val {
    let some: Map<Val, Val> = some
        .iter()
        .map(|(k, v)| {
            let ctx = v.ctx.generate();
            let input = v.input.generate();
            let pair = Val::Pair(Pair::new(ctx, input).into());
            (k.clone(), pair)
        })
        .collect();
    Val::Map(some.into())
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
                    error!("first {:?} should be a list", head_tail.first);
                    return None;
                };
                let head = parse_list_head(head)?;
                let tail = ParseMode::parse(head_tail.second)?;
                Some(ListMode { head, tail })
            }
            v => {
                error!("{v:?} should be a list, a pair or a symbol");
                None
            }
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
                let else_ = Pair::new(None, None);
                Some(MapMode { some, else_ })
            }
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    error!("first {:?} should be a map", some_else.first);
                    return None;
                };
                let some = parse_map_some(some)?;
                let else_ = parse_map_else(some_else.second)?;
                Some(MapMode { some, else_ })
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
        Val::Pair(else_) => {
            let else_ = Pair::from(else_);
            let key = ParseMode::parse(else_.first)?;
            let value = ParseMode::parse(else_.second)?;
            Pair::new(key, value)
        }
        v => {
            error!("{v:?} should be a pair, a symbol or a unit");
            return None;
        }
    };
    Some(mode)
}

impl GenerateMode<Val> for MapMode {
    fn generate(&self) -> Val {
        let some = generate_map_some(&self.some);
        if self.else_.first.is_none() && self.else_.second.is_none() {
            return some;
        }
        let else_ = generate_map_else(&self.else_.first, &self.else_.second);
        Val::Pair(Pair::new(some, else_).into())
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
