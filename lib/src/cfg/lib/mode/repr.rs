use const_format::concatcp;
use log::error;

use crate::cfg::mode::CompMode;
use crate::cfg::mode::ListMode;
use crate::cfg::mode::MapMode;
use crate::cfg::mode::Mode;
use crate::cfg::mode::PairMode;
use crate::cfg::mode::PrimMode;
use crate::cfg::mode::SymbolMode;
use crate::cfg::mode::TaskMode;
use crate::cfg::mode::TaskPrimMode;
use crate::cfg::utils::map_remove;
use crate::cfg::utils::symbol;
use crate::semantics::core::SYMBOL_EVAL_CHAR;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::val::LIST;
use crate::semantics::val::ListVal;
use crate::semantics::val::MAP;
use crate::semantics::val::MapVal;
use crate::semantics::val::PAIR;
use crate::semantics::val::TASK;
use crate::semantics::val::Val;
use crate::type_::Action;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;
use crate::type_::Unit;

pub(in crate::cfg) trait ParseMode<T>: Sized + Clone {
    fn parse(mode: T) -> Option<Self>;
}

pub(in crate::cfg) trait GenerateMode<T> {
    fn generate(&self) -> T;
}

pub(in crate::cfg) fn parse(mode: Val) -> Option<Mode> {
    Mode::parse(mode)
}

// todo design
#[expect(dead_code)]
pub(in crate::cfg) fn generate(mode: &Mode) -> Val {
    mode.generate()
}

impl<T: ParseMode<Val>> ParseMode<Val> for Box<T> {
    fn parse(mode: Val) -> Option<Self> {
        Some(Box::new(T::parse(mode)?))
    }
}

impl<T: GenerateMode<Val>> GenerateMode<Val> for Box<T> {
    fn generate(&self) -> Val {
        (**self).generate()
    }
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

impl ParseMode<Val> for Mode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Unit(unit) => Self::parse(unit),
            Val::Symbol(symbol) => Self::parse(symbol),
            Val::Map(map) => Self::parse(map),
            Val::Func(func) => Some(Mode::Func(func)),
            _ => None,
        }
    }
}

impl ParseMode<Unit> for Mode {
    fn parse(_: Unit) -> Option<Self> {
        Some(Self::id())
    }
}

impl ParseMode<Symbol> for Mode {
    fn parse(mode: Symbol) -> Option<Self> {
        Some(Self::from(PrimMode::parse(mode)?))
    }
}

impl ParseMode<MapVal> for Mode {
    fn parse(mode: MapVal) -> Option<Self> {
        Some(Mode::Comp(CompMode::parse(mode)?))
    }
}

impl GenerateMode<Val> for Mode {
    fn generate(&self) -> Val {
        match self {
            Mode::Comp(mode) => Val::Map(mode.generate()),
            Mode::Func(mode) => Val::Func(mode.clone()),
        }
    }
}

impl ParseMode<Val> for PrimMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Unit(_) => Some(PrimMode::id()),
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

impl ParseMode<Symbol> for PrimMode {
    fn parse(mode: Symbol) -> Option<Self> {
        let mode = match &*mode {
            FORM_ID => PrimMode::new(SymbolMode::Id, TaskPrimMode::Form),
            FORM_LITERAL => PrimMode::new(SymbolMode::Literal, TaskPrimMode::Form),
            FORM_REF => PrimMode::new(SymbolMode::Ref, TaskPrimMode::Form),
            FORM_EVAL => PrimMode::new(SymbolMode::Eval, TaskPrimMode::Form),
            EVAL_ID => PrimMode::new(SymbolMode::Id, TaskPrimMode::Eval),
            EVAL_LITERAL => PrimMode::new(SymbolMode::Literal, TaskPrimMode::Eval),
            EVAL_REF => PrimMode::new(SymbolMode::Ref, TaskPrimMode::Eval),
            EVAL_EVAL => PrimMode::new(SymbolMode::Eval, TaskPrimMode::Eval),
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
        if self.is_id() {
            return Val::default();
        }
        let s = match (self.task, self.symbol) {
            (TaskPrimMode::Form, SymbolMode::Id) => FORM_ID,
            (TaskPrimMode::Form, SymbolMode::Literal) => FORM_LITERAL,
            (TaskPrimMode::Form, SymbolMode::Ref) => FORM_REF,
            (TaskPrimMode::Form, SymbolMode::Eval) => FORM_EVAL,
            (TaskPrimMode::Eval, SymbolMode::Id) => EVAL_ID,
            (TaskPrimMode::Eval, SymbolMode::Literal) => EVAL_LITERAL,
            (TaskPrimMode::Eval, SymbolMode::Ref) => EVAL_REF,
            (TaskPrimMode::Eval, SymbolMode::Eval) => EVAL_EVAL,
        };
        symbol(s)
    }
}

const DEFAULT: &str = "default";

impl ParseMode<MapVal> for CompMode {
    fn parse(mut map: MapVal) -> Option<Self> {
        let default = ParseMode::parse(map_remove(&mut map, DEFAULT))?;
        let pair = ParseMode::parse(map_remove(&mut map, PAIR))?;
        let task = ParseMode::parse(map_remove(&mut map, TASK))?;
        let list = ParseMode::parse(map_remove(&mut map, LIST))?;
        let map = ParseMode::parse(map_remove(&mut map, MAP))?;
        Some(CompMode { default, pair, task, list, map })
    }
}

impl GenerateMode<MapVal> for CompMode {
    fn generate(&self) -> MapVal {
        let mut map = Map::default();
        let default = self.default.generate();
        if !default.is_unit() {
            map.insert(symbol(DEFAULT), default);
        }
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
            Val::Symbol(symbol) => {
                let mode = Mode::parse(symbol)?;
                Some(PairMode { some: Map::default(), first: mode.clone(), second: mode })
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

impl ParseMode<Val> for TaskMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(symbol) => {
                let mode = Mode::parse(symbol)?;
                Some(TaskMode { func: mode.clone(), ctx: mode.clone(), input: mode })
            }
            Val::Task(task) => {
                let task = Task::from(task);
                let func = ParseMode::parse(task.func)?;
                let ctx = ParseMode::parse(task.ctx)?;
                let input = ParseMode::parse(task.input)?;
                Some(TaskMode { func, ctx, input })
            }
            v => {
                error!("{v:?} should be a task, a pair or a symbol");
                None
            }
        }
    }
}

impl GenerateMode<Val> for TaskMode {
    fn generate(&self) -> Val {
        let action = Action::Call;
        let func = GenerateMode::generate(&self.func);
        let ctx = GenerateMode::generate(&self.ctx);
        let input = GenerateMode::generate(&self.input);
        Val::Task(Task { action, func, ctx, input }.into())
    }
}

impl ParseMode<Val> for ListMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(symbol) => {
                let mode = Mode::parse(symbol)?;
                Some(ListMode { head: List::default(), tail: mode })
            }
            Val::List(head) => {
                let head = parse_list_head(head)?;
                let tail = Mode::id();
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

fn parse_list_head(head: ListVal) -> Option<List<Mode>> {
    List::from(head).into_iter().map(ParseMode::parse).collect()
}

impl GenerateMode<Val> for ListMode {
    fn generate(&self) -> Val {
        let head = generate_list_head(&self.head);
        if self.tail.is_id() {
            return head;
        }
        let tail = GenerateMode::generate(&self.tail);
        Val::Pair(Pair::new(head, tail).into())
    }
}

fn generate_list_head(head: &List<Mode>) -> Val {
    let head: List<Val> = head.iter().map(GenerateMode::generate).collect();
    Val::List(head.into())
}

impl ParseMode<Val> for MapMode {
    fn parse(mode: Val) -> Option<Self> {
        match mode {
            Val::Symbol(symbol) => {
                let mode = Mode::parse(symbol)?;
                Some(MapMode { some: Map::default(), else_: mode })
            }
            Val::Map(some) => {
                let some = parse_map_some(some)?;
                let else_ = Mode::id();
                Some(MapMode { some, else_ })
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

fn parse_map_some(some: MapVal) -> Option<Map<Val, Mode>> {
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
        if self.else_.is_id() {
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
