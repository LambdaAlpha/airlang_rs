use log::error;

use super::repr::OptBinding;
use super::repr::parse_contract;
use crate::prelude::utils::symbol;
use crate::semantics::core::SYMBOL_LITERAL_CHAR;
use crate::semantics::core::SYMBOL_REF_CHAR;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::CallVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::syntax::CALL_FORWARD;
use crate::syntax::CALL_REVERSE;
use crate::type_::Call;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Unit;

pub(in crate::prelude) enum Pattern {
    Any(OptBinding),
    Val(Val),
    Pair(Box<Pair<Pattern, Pattern>>),
    Call(Box<Call<Pattern, Pattern>>),
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
        Val::Call(call) => parse_call(ctx, call),
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

fn parse_call(ctx: PatternCtx, call: CallVal) -> Option<Pattern> {
    let call = Call::from(call);
    if call.reverse {
        return parse_call1(ctx, call.reverse, call.func, call.input);
    }
    match call.func {
        Val::Unit(_) => parse_with_guard(ctx, call.input),
        Val::Symbol(symbol) => match &*symbol {
            CALL_FORWARD | CALL_REVERSE => {
                let Val::Pair(pair) = call.input else {
                    error!("{:?} should be a pair", call.input);
                    return None;
                };
                let pair = Pair::from(pair);
                parse_call1(ctx, &*symbol == CALL_REVERSE, pair.first, pair.second)
            }
            s => {
                error!("{s} should be one of {CALL_FORWARD} or {CALL_REVERSE}");
                None
            }
        },
        func => parse_call1(ctx, false, func, call.input),
    }
}

fn parse_call1(ctx: PatternCtx, reverse: bool, func: Val, input: Val) -> Option<Pattern> {
    let func = parse_pattern(ctx, func)?;
    let input = parse_pattern(ctx, input)?;
    Some(Pattern::Call(Box::new(Call::new(reverse, func, input))))
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
        Pattern::Call(call) => match_call(call, val),
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

fn match_call(pattern: &Call<Pattern, Pattern>, val: &Val) -> bool {
    let Val::Call(val) = val else {
        error!("{val:?} should be a call");
        return false;
    };
    let func = match_pattern(&pattern.func, &val.func);
    let input = match_pattern(&pattern.input, &val.input);
    func && input
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
        Pattern::Call(call) => assign_call(ctx, *call, val),
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

fn assign_call(ctx: &mut Ctx, pattern: Call<Pattern, Pattern>, val: Val) -> Val {
    let Val::Call(val) = val else {
        error!("{val:?} should be a call");
        return Val::default();
    };
    if pattern.reverse != val.reverse {
        error!("reverse should be equal");
        return Val::default();
    }
    let val = Call::from(val);
    let func = assign_pattern(ctx, pattern.func, val.func);
    let input = assign_pattern(ctx, pattern.input, val.input);
    Val::Call(Call::new(val.reverse, func, input).into())
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
