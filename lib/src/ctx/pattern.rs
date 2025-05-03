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
};

pub(crate) enum Pattern {
    Any(Binding),
    Pair(Box<Pair<Pattern, Pattern>>),
    Call(Box<Call<Pattern, Pattern>>),
    List(List<Pattern>),
    Map(Map<Val, Pattern>),
}

#[derive(Copy, Clone)]
pub(crate) struct PatternCtx {
    pub(crate) extra: Extra,
    pub(crate) allow_extra: bool,
}

pub(crate) fn parse_pattern(ctx: PatternCtx, pattern: Val) -> Option<Pattern> {
    match pattern {
        Val::Symbol(name) => Some(parse_any(ctx, name)),
        Val::Pair(pair) => parse_pair(ctx, pair),
        Val::Call(call) => {
            if ctx.allow_extra && call.func.is_unit() {
                parse_with_extra(ctx, call)
            } else {
                parse_call(ctx, call)
            }
        }
        Val::List(list) => parse_list(ctx, list),
        Val::Map(map) => parse_map(ctx, map),
        _ => None,
    }
}

fn parse_any(ctx: PatternCtx, name: Symbol) -> Pattern {
    Pattern::Any(Binding { name, extra: ctx.extra })
}

fn parse_pair(mut ctx: PatternCtx, pair: PairVal) -> Option<Pattern> {
    ctx.allow_extra = true;
    let pair = Pair::from(pair);
    let first = parse_pattern(ctx, pair.first)?;
    let second = parse_pattern(ctx, pair.second)?;
    Some(Pattern::Pair(Box::new(Pair::new(first, second))))
}

fn parse_call(mut ctx: PatternCtx, call: CallVal) -> Option<Pattern> {
    ctx.allow_extra = true;
    let call = Call::from(call);
    let func = parse_pattern(ctx, call.func)?;
    let input = parse_pattern(ctx, call.input)?;
    Some(Pattern::Call(Box::new(Call::new(func, input))))
}

fn parse_list(mut ctx: PatternCtx, list: ListVal) -> Option<Pattern> {
    ctx.allow_extra = true;
    let list = List::from(list);
    let list =
        list.into_iter().map(|item| parse_pattern(ctx, item)).collect::<Option<List<_>>>()?;
    Some(Pattern::List(list))
}

fn parse_map(mut ctx: PatternCtx, map: MapVal) -> Option<Pattern> {
    ctx.allow_extra = true;
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

fn parse_with_extra(mut ctx: PatternCtx, call: CallVal) -> Option<Pattern> {
    ctx.allow_extra = false;
    let call = Call::from(call);
    let Val::Pair(pair) = call.input else {
        return None;
    };
    let pair = Pair::from(pair);
    ctx.extra = parse_extra(pair.second, ctx.extra)?;
    if ctx.extra.static1 {
        return None;
    }
    parse_pattern(ctx, pair.first)
}

pub(crate) fn assign_pattern(ctx: MutFnCtx, pattern: Pattern, val: Val) -> Val {
    match pattern {
        Pattern::Any(binding) => assign_any(ctx, binding, val),
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
