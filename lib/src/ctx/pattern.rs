use crate::{
    Call,
    CallVal,
    List,
    ListVal,
    Map,
    MapVal,
    MutFnCtx,
    Pair,
    PairVal,
    Symbol,
    Unit,
    Val,
    ctx::{
        map::{
            CtxMapRef,
            CtxValue,
        },
        ref1::CtxRef,
        repr::{
            Binding,
            Extra,
            parse_extra,
        },
    },
    mode::symbol::{
        LITERAL_CHAR,
        REF_CHAR,
    },
    syntax::CALL,
    utils::val::symbol,
};

pub(crate) enum Pattern {
    Any(Binding),
    Val(Val),
    Pair(Box<Pair<Pattern, Pattern>>),
    Call(Box<Call<Pattern, Pattern>>),
    List(List<Pattern>),
    Map(Map<Val, Pattern>),
}

#[derive(Default, Copy, Clone)]
pub(crate) struct PatternCtx {
    pub(crate) extra: Extra,
}

pub(crate) fn parse_pattern(ctx: PatternCtx, pattern: Val) -> Option<Pattern> {
    match pattern {
        Val::Symbol(symbol) => parse_symbol(ctx, symbol),
        Val::Pair(pair) => parse_pair(ctx, pair),
        Val::List(list) => parse_list(ctx, list),
        Val::Map(map) => parse_map(ctx, map),
        Val::Call(call) => parse_call(ctx, call),
        val => Some(Pattern::Val(val)),
    }
}

fn parse_symbol(ctx: PatternCtx, s: Symbol) -> Option<Pattern> {
    let pattern = match s.chars().next() {
        Some(LITERAL_CHAR) => Pattern::Val(symbol(&s[1 ..])),
        Some(REF_CHAR) => {
            let name = Symbol::from_str(&s[1 ..]);
            Pattern::Any(Binding { name, extra: ctx.extra })
        }
        _ => Pattern::Any(Binding { name: s, extra: ctx.extra }),
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
    match call.func {
        Val::Unit(_) => parse_with_extra(ctx, call.input),
        Val::Symbol(symbol) => match &*symbol {
            CALL => {
                let Val::Pair(pair) = call.input else {
                    return None;
                };
                let pair = Pair::from(pair);
                parse_call1(ctx, pair.first, pair.second)
            }
            _ => None,
        },
        func => parse_call1(ctx, func, call.input),
    }
}

fn parse_call1(ctx: PatternCtx, func: Val, input: Val) -> Option<Pattern> {
    let func = parse_pattern(ctx, func)?;
    let input = parse_pattern(ctx, input)?;
    Some(Pattern::Call(Box::new(Call::new(func, input))))
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

fn parse_with_extra(mut ctx: PatternCtx, val: Val) -> Option<Pattern> {
    let Val::Pair(pair) = val else {
        return None;
    };
    let pair = Pair::from(pair);
    ctx.extra = parse_extra(pair.second, ctx.extra)?;
    if ctx.extra.static1 {
        return None;
    }
    parse_pattern(ctx, pair.first)
}

pub(crate) fn match_pattern(pattern: &Pattern, val: &Val) -> bool {
    match pattern {
        Pattern::Any(binding) => match_any(binding, val),
        Pattern::Val(expected) => match_val(expected, val),
        Pattern::Pair(pair) => match_pair(pair, val),
        Pattern::Call(call) => match_call(call, val),
        Pattern::List(list) => match_list(list, val),
        Pattern::Map(map) => match_map(map, val),
    }
}

fn match_any(_binding: &Binding, _val: &Val) -> bool {
    true
}

fn match_val(expected: &Val, val: &Val) -> bool {
    *expected == *val
}

fn match_pair(pattern: &Pair<Pattern, Pattern>, val: &Val) -> bool {
    let Val::Pair(val) = val else {
        return false;
    };
    let first = match_pattern(&pattern.first, &val.first);
    let second = match_pattern(&pattern.second, &val.second);
    first && second
}

fn match_call(pattern: &Call<Pattern, Pattern>, val: &Val) -> bool {
    let Val::Call(val) = val else {
        return false;
    };
    let func = match_pattern(&pattern.func, &val.func);
    let input = match_pattern(&pattern.input, &val.input);
    func && input
}

fn match_list(pattern: &List<Pattern>, val: &Val) -> bool {
    let Val::List(val) = val else {
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
        return false;
    };
    pattern.iter().all(|(k, pattern)| {
        let val = val.get(k).unwrap_or(&Val::Unit(Unit));
        match_pattern(pattern, val)
    })
}

pub(crate) fn assign_pattern(ctx: MutFnCtx, pattern: Pattern, val: Val) -> Val {
    match pattern {
        Pattern::Any(binding) => assign_any(ctx, binding, val),
        Pattern::Val(expected) => assign_val(ctx, expected, val),
        Pattern::Pair(pair) => assign_pair(ctx, *pair, val),
        Pattern::Call(call) => assign_call(ctx, *call, val),
        Pattern::List(list) => assign_list(ctx, list, val),
        Pattern::Map(map) => assign_map(ctx, map, val),
    }
}

fn assign_any(ctx: MutFnCtx, binding: Binding, val: Val) -> Val {
    let ctx_value = CtxValue { val, access: binding.extra.access, static1: false };
    let Ok(ctx) = ctx.get_variables_mut() else {
        return Val::default();
    };
    let Ok(last) = ctx.put_value(binding.name, ctx_value) else {
        return Val::default();
    };
    last.unwrap_or_default()
}

fn assign_val(_ctx: MutFnCtx, _expected: Val, _val: Val) -> Val {
    Val::default()
}

fn assign_pair(mut ctx: MutFnCtx, pattern: Pair<Pattern, Pattern>, val: Val) -> Val {
    let Val::Pair(val) = val else {
        return Val::default();
    };
    let val = Pair::from(val);
    let first = assign_pattern(ctx.reborrow(), pattern.first, val.first);
    let second = assign_pattern(ctx, pattern.second, val.second);
    Val::Pair(Pair::new(first, second).into())
}

fn assign_call(mut ctx: MutFnCtx, pattern: Call<Pattern, Pattern>, val: Val) -> Val {
    let Val::Call(val) = val else {
        return Val::default();
    };
    let val = Call::from(val);
    let func = assign_pattern(ctx.reborrow(), pattern.func, val.func);
    let input = assign_pattern(ctx, pattern.input, val.input);
    Val::Call(Call::new(func, input).into())
}

fn assign_list(mut ctx: MutFnCtx, pattern: List<Pattern>, val: Val) -> Val {
    let Val::List(val) = val else {
        return Val::default();
    };
    let mut list = List::from(Vec::with_capacity(pattern.len()));
    let mut val_iter = List::from(val).into_iter();
    for p in pattern {
        let val = val_iter.next().unwrap_or_default();
        let last_val = assign_pattern(ctx.reborrow(), p, val);
        list.push(last_val);
    }
    Val::List(list.into())
}

fn assign_map(mut ctx: MutFnCtx, pattern: Map<Val, Pattern>, val: Val) -> Val {
    let Val::Map(mut val) = val else {
        return Val::default();
    };
    let map: Map<Val, Val> = pattern
        .into_iter()
        .map(|(k, pattern)| {
            let val = val.remove(&k).unwrap_or_default();
            let last_val = assign_pattern(ctx.reborrow(), pattern, val);
            (k, last_val)
        })
        .collect();
    Val::Map(map.into())
}
