use std::mem::swap;

use log::error;

use crate::prelude::utils::symbol;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;
use crate::type_::Unit;

pub(in crate::prelude) enum Pattern {
    Any(Symbol),
    Val(Val),
    Pair(Box<Pair<Pattern, Pattern>>),
    Task(Box<Task<Pattern, Pattern, Pattern>>),
    List(List<Pattern>),
    Map(Map<Val, Pattern>),
}

pub(in crate::prelude) trait PatternParse {
    fn parse(self) -> Option<Pattern>;
}

impl PatternParse for Val {
    fn parse(self) -> Option<Pattern> {
        match self {
            Val::Symbol(symbol) => symbol.parse(),
            Val::Pair(pair) => pair.parse(),
            Val::List(list) => list.parse(),
            Val::Map(map) => map.parse(),
            Val::Task(task) => task.parse(),
            val => Some(Pattern::Val(val)),
        }
    }
}

const SYMBOL_LITERAL_CHAR: char = '*';
const SYMBOL_REF_CHAR: char = '#';

impl PatternParse for Symbol {
    fn parse(self) -> Option<Pattern> {
        let pattern = match self.chars().next() {
            Some(SYMBOL_LITERAL_CHAR) => Pattern::Val(symbol(&self[1 ..])),
            Some(SYMBOL_REF_CHAR) => Pattern::Any(Symbol::from_str_unchecked(&self[1 ..])),
            _ => Pattern::Any(self),
        };
        Some(pattern)
    }
}

impl PatternParse for PairVal {
    fn parse(self) -> Option<Pattern> {
        let pair = Pair::from(self);
        let first = pair.first.parse()?;
        let second = pair.second.parse()?;
        Some(Pattern::Pair(Box::new(Pair::new(first, second))))
    }
}

impl PatternParse for TaskVal {
    fn parse(self) -> Option<Pattern> {
        let task = Task::from(self);
        let func = task.func.parse()?;
        let ctx = task.ctx.parse()?;
        let input = task.input.parse()?;
        Some(Pattern::Task(Box::new(Task { action: task.action, func, ctx, input })))
    }
}

impl PatternParse for ListVal {
    fn parse(self) -> Option<Pattern> {
        let list = List::from(self);
        let list = list.into_iter().map(PatternParse::parse).collect::<Option<List<_>>>()?;
        Some(Pattern::List(list))
    }
}

impl PatternParse for MapVal {
    fn parse(self) -> Option<Pattern> {
        let map = Map::from(self);
        let map =
            map.into_iter().map(|(k, v)| Some((k, v.parse()?))).collect::<Option<Map<_, _>>>()?;
        Some(Pattern::Map(map))
    }
}

pub(in crate::prelude) trait PatternMatch<T> {
    fn match_(&self, val: &T) -> bool;
}

impl PatternMatch<Val> for Pattern {
    fn match_(&self, val: &Val) -> bool {
        match self {
            Pattern::Any(name) => name.match_(val),
            Pattern::Val(expected) => expected.match_(val),
            Pattern::Pair(pair) => pair.match_(val),
            Pattern::Task(task) => task.match_(val),
            Pattern::List(list) => list.match_(val),
            Pattern::Map(map) => map.match_(val),
        }
    }
}

impl PatternMatch<Val> for Symbol {
    fn match_(&self, _val: &Val) -> bool {
        true
    }
}

impl PatternMatch<Val> for Val {
    fn match_(&self, val: &Val) -> bool {
        *self == *val
    }
}

impl PatternMatch<Val> for Pair<Pattern, Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::Pair(val) = val else {
            error!("{val:?} should be a pair");
            return false;
        };
        let first = self.first.match_(&val.first);
        let second = self.second.match_(&val.second);
        first && second
    }
}

impl PatternMatch<Val> for Task<Pattern, Pattern, Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::Task(val) = val else {
            error!("{val:?} should be a task");
            return false;
        };
        let func = self.func.match_(&val.func);
        let ctx = self.ctx.match_(&val.ctx);
        let input = self.input.match_(&val.input);
        func && ctx && input
    }
}

impl PatternMatch<Val> for List<Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::List(val) = val else {
            error!("{val:?} should be a list");
            return false;
        };
        let mut val_iter = val.iter();
        self.iter().all(|p| {
            let val = val_iter.next().unwrap_or(&Val::Unit(Unit));
            p.match_(val)
        })
    }
}

impl PatternMatch<Val> for Map<Val, Pattern> {
    fn match_(&self, val: &Val) -> bool {
        let Val::Map(val) = val else {
            error!("{val:?} should be a map");
            return false;
        };
        self.iter().all(|(k, pattern)| {
            let val = val.get(k).unwrap_or(&Val::Unit(Unit));
            pattern.match_(val)
        })
    }
}

pub(in crate::prelude) trait PatternAssign<Ctx, Val> {
    fn assign(self, ctx: &mut Ctx, val: Val) -> Val;
}

impl PatternAssign<Val, Val> for Pattern {
    fn assign(self, ctx: &mut Val, val: Val) -> Val {
        match self {
            Pattern::Any(name) => name.assign(ctx, val),
            Pattern::Val(expected) => expected.assign(ctx, val),
            Pattern::Pair(pair) => pair.assign(ctx, val),
            Pattern::Task(task) => task.assign(ctx, val),
            Pattern::List(list) => list.assign(ctx, val),
            Pattern::Map(map) => map.assign(ctx, val),
        }
    }
}

impl PatternAssign<Val, Val> for Symbol {
    fn assign(self, ctx: &mut Val, mut val: Val) -> Val {
        match ctx {
            Val::Ctx(ctx) => {
                let Ok(last) = ctx.put(self.clone(), val, Contract::None) else {
                    error!("variable {self:?} is not assignable");
                    return Val::default();
                };
                last.unwrap_or_default()
            }
            ctx => {
                let Some(cur_val) = ctx.ref_(self.clone()) else {
                    error!("variable {self:?} doesn't exist");
                    return Val::default();
                };
                if cur_val.is_const() {
                    error!("variable {self:?} should be mutable");
                    return Val::default();
                }
                swap(cur_val.unwrap(), &mut val);
                val
            }
        }
    }
}

impl PatternAssign<Val, Val> for Val {
    fn assign(self, _ctx: &mut Val, _val: Val) -> Val {
        Val::default()
    }
}

impl PatternAssign<Val, Val> for Pair<Pattern, Pattern> {
    fn assign(self, ctx: &mut Val, val: Val) -> Val {
        let Val::Pair(val) = val else {
            error!("{val:?} should be a pair");
            return Val::default();
        };
        let val = Pair::from(val);
        let first = self.first.assign(ctx, val.first);
        let second = self.second.assign(ctx, val.second);
        Val::Pair(Pair::new(first, second).into())
    }
}

impl PatternAssign<Val, Val> for Task<Pattern, Pattern, Pattern> {
    fn assign(self, c: &mut Val, val: Val) -> Val {
        let Val::Task(val) = val else {
            error!("{val:?} should be a task");
            return Val::default();
        };
        if self.action != val.action {
            error!("action should be equal");
            return Val::default();
        }
        let val = Task::from(val);
        let func = self.func.assign(c, val.func);
        let ctx = self.ctx.assign(c, val.ctx);
        let input = self.input.assign(c, val.input);
        Val::Task(Task { action: val.action, func, ctx, input }.into())
    }
}

impl PatternAssign<Val, Val> for List<Pattern> {
    fn assign(self, ctx: &mut Val, val: Val) -> Val {
        let Val::List(val) = val else {
            error!("{val:?} should be a list");
            return Val::default();
        };
        let mut list = List::from(Vec::with_capacity(self.len()));
        let mut val_iter = List::from(val).into_iter();
        for p in self {
            let val = val_iter.next().unwrap_or_default();
            let last_val = p.assign(ctx, val);
            list.push(last_val);
        }
        Val::List(list.into())
    }
}

impl PatternAssign<Val, Val> for Map<Val, Pattern> {
    fn assign(self, ctx: &mut Val, val: Val) -> Val {
        let Val::Map(mut val) = val else {
            error!("{val:?} should be a map");
            return Val::default();
        };
        let map: Map<Val, Val> = self
            .into_iter()
            .map(|(k, pattern)| {
                let val = val.remove(&k).unwrap_or_default();
                let last_val = pattern.assign(ctx, val);
                (k, last_val)
            })
            .collect();
        Val::Map(map.into())
    }
}
