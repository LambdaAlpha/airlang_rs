use log::error;

use super::repr::OptBinding;
use super::repr::parse_contract;
use crate::prelude::utils::symbol;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::syntax::CALL;
use crate::syntax::SOLVE;
use crate::type_::Action;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;
use crate::type_::Unit;

pub(in crate::prelude) enum Pattern {
    Any(OptBinding),
    Val(Val),
    Pair(Box<Pair<Pattern, Pattern>>),
    Task(Box<Task<Pattern, Pattern, Pattern>>),
    List(List<Pattern>),
    Map(Map<Val, Pattern>),
}

#[derive(Default, Copy, Clone)]
pub(in crate::prelude) struct PatternCtx {
    pub(in crate::prelude) contract: Option<Contract>,
}

pub(in crate::prelude) fn parse_pattern(ctx: PatternCtx, pattern: Val) -> Option<Pattern> {
    match pattern {
        Val::Symbol(symbol) => parse_symbol(ctx, symbol),
        Val::Pair(pair) => parse_pair(ctx, pair),
        Val::List(list) => parse_list(ctx, list),
        Val::Map(map) => parse_map(ctx, map),
        Val::Task(task) => parse_task(ctx, task),
        val => Some(Pattern::Val(val)),
    }
}

// todo design
fn parse_symbol(ctx: PatternCtx, s: Symbol) -> Option<Pattern> {
    let pattern = match s.chars().next() {
        Some(SYMBOL_LITERAL_CHAR) => Pattern::Val(symbol(&s[1 ..])),
        Some(SYMBOL_REF_CHAR) => {
            let name = Symbol::from_str_unchecked(&s[1 ..]);
            Pattern::Any(OptBinding { name, contract: ctx.contract })
        }
        _ => Pattern::Any(OptBinding { name: s, contract: ctx.contract }),
    };
    Some(pattern)
}

fn parse_pair(ctx: PatternCtx, pair: PairVal) -> Option<Pattern> {
    let pair = Pair::from(pair);
    let first = parse_pattern(ctx, pair.first)?;
    let second = parse_pattern(ctx, pair.second)?;
    Some(Pattern::Pair(Box::new(Pair::new(first, second))))
}

fn parse_task(ctx: PatternCtx, task: TaskVal) -> Option<Pattern> {
    let task = Task::from(task);
    if task.action != Action::Call {
        return parse_task_struct(ctx, task.action, task.func, task.ctx, task.input);
    }
    match task.func {
        Val::Unit(_) => parse_with_guard(ctx, task.input),
        Val::Symbol(symbol) => match &*symbol {
            CALL | SOLVE => {
                let Val::Pair(pair) = task.input else {
                    error!("{:?} should be a pair", task.input);
                    return None;
                };
                let pair = Pair::from(pair);
                let action = if &*symbol == SOLVE { Action::Solve } else { Action::Call };
                parse_task_struct(ctx, action, pair.first, task.ctx, pair.second)
            }
            s => {
                error!("{s} should be one of {CALL} or {SOLVE}");
                None
            }
        },
        func => parse_task_struct(ctx, Action::Call, func, task.ctx, task.input),
    }
}

fn parse_task_struct(
    c: PatternCtx, action: Action, func: Val, ctx: Val, input: Val,
) -> Option<Pattern> {
    let func = parse_pattern(c, func)?;
    let ctx = parse_pattern(c, ctx)?;
    let input = parse_pattern(c, input)?;
    Some(Pattern::Task(Box::new(Task { action, func, ctx, input })))
}

fn parse_list(ctx: PatternCtx, list: ListVal) -> Option<Pattern> {
    let list = List::from(list);
    let list =
        list.into_iter().map(|item| parse_pattern(ctx, item)).collect::<Option<List<_>>>()?;
    Some(Pattern::List(list))
}

fn parse_map(ctx: PatternCtx, map: MapVal) -> Option<Pattern> {
    let map = Map::from(map);
    let map = map
        .into_iter()
        .map(|(k, v)| {
            let v = parse_pattern(ctx, v)?;
            Some((k, v))
        })
        .collect::<Option<Map<_, _>>>()?;
    Some(Pattern::Map(map))
}

fn parse_with_guard(mut ctx: PatternCtx, val: Val) -> Option<Pattern> {
    let Val::Pair(pair) = val else {
        error!("{val:?} should be a pair");
        return None;
    };
    let pair = Pair::from(pair);
    ctx.contract = parse_contract(pair.second);
    parse_pattern(ctx, pair.first)
}

pub(in crate::prelude) fn match_pattern(pattern: &Pattern, val: &Val) -> bool {
    match pattern {
        Pattern::Any(binding) => match_any(binding, val),
        Pattern::Val(expected) => match_val(expected, val),
        Pattern::Pair(pair) => match_pair(pair, val),
        Pattern::Task(task) => match_task(task, val),
        Pattern::List(list) => match_list(list, val),
        Pattern::Map(map) => match_map(map, val),
    }
}

fn match_any(_binding: &OptBinding, _val: &Val) -> bool {
    true
}

fn match_val(expected: &Val, val: &Val) -> bool {
    *expected == *val
}

fn match_pair(pattern: &Pair<Pattern, Pattern>, val: &Val) -> bool {
    let Val::Pair(val) = val else {
        error!("{val:?} should be a pair");
        return false;
    };
    let first = match_pattern(&pattern.first, &val.first);
    let second = match_pattern(&pattern.second, &val.second);
    first && second
}

fn match_task(pattern: &Task<Pattern, Pattern, Pattern>, val: &Val) -> bool {
    let Val::Task(val) = val else {
        error!("{val:?} should be a task");
        return false;
    };
    let func = match_pattern(&pattern.func, &val.func);
    let ctx = match_pattern(&pattern.ctx, &val.ctx);
    let input = match_pattern(&pattern.input, &val.input);
    func && ctx && input
}

fn match_list(pattern: &List<Pattern>, val: &Val) -> bool {
    let Val::List(val) = val else {
        error!("{val:?} should be a list");
        return false;
    };
    let mut val_iter = val.iter();
    pattern.iter().all(|p| {
        let val = val_iter.next().unwrap_or(&Val::Unit(Unit));
        match_pattern(p, val)
    })
}

fn match_map(pattern: &Map<Val, Pattern>, val: &Val) -> bool {
    let Val::Map(val) = val else {
        error!("{val:?} should be a map");
        return false;
    };
    pattern.iter().all(|(k, pattern)| {
        let val = val.get(k).unwrap_or(&Val::Unit(Unit));
        match_pattern(pattern, val)
    })
}

pub(in crate::prelude) fn assign_pattern(ctx: &mut Ctx, pattern: Pattern, val: Val) -> Val {
    match pattern {
        Pattern::Any(binding) => assign_any(ctx, binding, val),
        Pattern::Val(expected) => assign_val(ctx, expected, val),
        Pattern::Pair(pair) => assign_pair(ctx, *pair, val),
        Pattern::Task(task) => assign_task(ctx, *task, val),
        Pattern::List(list) => assign_list(ctx, list, val),
        Pattern::Map(map) => assign_map(ctx, map, val),
    }
}

fn assign_any(ctx: &mut Ctx, binding: OptBinding, val: Val) -> Val {
    let Ok(last) =
        ctx.variables_mut().put(binding.name.clone(), val, binding.contract.unwrap_or_default())
    else {
        error!("variable {:?} is not assignable", binding.name);
        return Val::default();
    };
    last.unwrap_or_default()
}

fn assign_val(_ctx: &mut Ctx, _expected: Val, _val: Val) -> Val {
    Val::default()
}

fn assign_pair(ctx: &mut Ctx, pattern: Pair<Pattern, Pattern>, val: Val) -> Val {
    let Val::Pair(val) = val else {
        error!("{val:?} should be a pair");
        return Val::default();
    };
    let val = Pair::from(val);
    let first = assign_pattern(ctx, pattern.first, val.first);
    let second = assign_pattern(ctx, pattern.second, val.second);
    Val::Pair(Pair::new(first, second).into())
}

fn assign_task(c: &mut Ctx, pattern: Task<Pattern, Pattern, Pattern>, val: Val) -> Val {
    let Val::Task(val) = val else {
        error!("{val:?} should be a task");
        return Val::default();
    };
    if pattern.action != val.action {
        error!("action should be equal");
        return Val::default();
    }
    let val = Task::from(val);
    let func = assign_pattern(c, pattern.func, val.func);
    let ctx = assign_pattern(c, pattern.ctx, val.ctx);
    let input = assign_pattern(c, pattern.input, val.input);
    Val::Task(Task { action: val.action, func, ctx, input }.into())
}

fn assign_list(ctx: &mut Ctx, pattern: List<Pattern>, val: Val) -> Val {
    let Val::List(val) = val else {
        error!("{val:?} should be a list");
        return Val::default();
    };
    let mut list = List::from(Vec::with_capacity(pattern.len()));
    let mut val_iter = List::from(val).into_iter();
    for p in pattern {
        let val = val_iter.next().unwrap_or_default();
        let last_val = assign_pattern(ctx, p, val);
        list.push(last_val);
    }
    Val::List(list.into())
}

fn assign_map(ctx: &mut Ctx, pattern: Map<Val, Pattern>, val: Val) -> Val {
    let Val::Map(mut val) = val else {
        error!("{val:?} should be a map");
        return Val::default();
    };
    let map: Map<Val, Val> = pattern
        .into_iter()
        .map(|(k, pattern)| {
            let val = val.remove(&k).unwrap_or_default();
            let last_val = assign_pattern(ctx, pattern, val);
            (k, last_val)
        })
        .collect();
    Val::Map(map.into())
}
