use const_format::concatcp;
use log::error;

use super::CodeMode;
use super::CompMode;
use super::DataMode;
use super::ListMode;
use super::MapMode;
use super::Mode;
use super::PairMode;
use super::PrimMode;
use super::SymbolMode;
use super::TaskMode;
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
use crate::semantics::val::LIST;
use crate::semantics::val::ListVal;
use crate::semantics::val::MAP;
use crate::semantics::val::MapVal;
use crate::semantics::val::PAIR;
use crate::semantics::val::SYMBOL;
use crate::semantics::val::TASK;
use crate::semantics::val::Val;
use crate::type_::Action;
use crate::type_::Bit;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;

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
                    Val::Bit(b) => *b,
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
        let task = ParseMode::parse(map_remove(&mut map, TASK))?;
        let list = ParseMode::parse(map_remove(&mut map, LIST))?;
        let map = ParseMode::parse(map_remove(&mut map, MAP))?;
        Some(PrimMode { symbol, pair, task, list, map })
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
            FORM_LITERAL => PrimMode::symbol_task(SymbolMode::Literal, CodeMode::Form),
            FORM_REF => PrimMode::symbol_task(SymbolMode::Ref, CodeMode::Form),
            FORM_MOVE => PrimMode::symbol_task(SymbolMode::Move, CodeMode::Form),
            FORM_EVAL => PrimMode::symbol_task(SymbolMode::Eval, CodeMode::Form),
            EVAL_LITERAL => PrimMode::symbol_task(SymbolMode::Literal, CodeMode::Eval),
            EVAL_REF => PrimMode::symbol_task(SymbolMode::Ref, CodeMode::Eval),
            EVAL_MOVE => PrimMode::symbol_task(SymbolMode::Move, CodeMode::Eval),
            EVAL_EVAL => PrimMode::symbol_task(SymbolMode::Eval, CodeMode::Eval),
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
            && self.task.is_none()
            && self.list.is_none()
            && self.map.is_none()
        {
            return Val::default();
        }

        if self.symbol.is_some()
            && self.pair.is_some()
            && self.task.is_some()
            && self.list.is_some()
            && self.map.is_some()
        {
            let s = match (self.task.unwrap(), self.symbol.unwrap()) {
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
        put_some(&mut map, TASK, &self.task);
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
        let task = ParseMode::parse(map_remove(&mut map, TASK))?;
        let list = ParseMode::parse(map_remove(&mut map, LIST))?;
        let map = ParseMode::parse(map_remove(&mut map, MAP))?;
        Some(CompMode { symbol, pair, task, list, map })
    }
}

impl GenerateMode<MapVal> for CompMode {
    fn generate(&self) -> MapVal {
        let mut map = Map::default();
        put_some(&mut map, SYMBOL, &self.symbol);
        put_some(&mut map, PAIR, &self.pair);
        put_some(&mut map, TASK, &self.task);
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
                let first = ParseMode::parse(pair.first)?;
                let second = ParseMode::parse(pair.second)?;
                Some(PairMode { some, first, second })
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
        let some = generate_map_some(&self.some);
        let first = GenerateMode::generate(&self.first);
        let second = GenerateMode::generate(&self.second);
        let else_ = Val::Pair(Pair::new(first, second).into());
        Val::Pair(Pair::new(some, else_).into())
    }
}

// todo design
impl ParseMode<Val> for TaskMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(s) => Some(Self::try_from(PrimMode::parse(s)?).unwrap()),
            Val::Task(task) => {
                let task = Task::from(task);
                // todo design
                let code = match task.action {
                    Action::Call => CodeMode::Eval,
                    Action::Solve => CodeMode::Form,
                };
                let func = ParseMode::parse(task.func)?;
                let ctx = ParseMode::parse(task.ctx)?;
                let input = ParseMode::parse(task.input)?;
                Some(TaskMode { code, func, ctx, input })
            }
            v => {
                error!("{v:?} should be a task, a pair or a symbol");
                None
            }
        }
    }
}

// todo design
impl GenerateMode<Val> for TaskMode {
    fn generate(&self) -> Val {
        let func = GenerateMode::generate(&self.func);
        let ctx = GenerateMode::generate(&self.ctx);
        let input = GenerateMode::generate(&self.input);
        // todo design
        let action = match self.code {
            CodeMode::Form => Action::Solve,
            CodeMode::Eval => Action::Call,
        };
        Val::Task(Task { action, func, ctx, input }.into())
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
                Some(MapMode { some, else_: None })
            }
            Val::Pair(some_else) => {
                let some_else = Pair::from(some_else);
                let Val::Map(some) = some_else.first else {
                    error!("first {:?} should be a map", some_else.first);
                    return None;
                };
                let some = parse_map_some(some)?;
                let else_ = ParseMode::parse(some_else.second)?;
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

impl GenerateMode<Val> for MapMode {
    fn generate(&self) -> Val {
        let some = generate_map_some(&self.some);
        if self.else_.is_none() {
            return some;
        }
        let else_ = self.else_.generate();
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
