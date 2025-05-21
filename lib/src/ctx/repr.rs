use crate::Bit;
use crate::Call;
use crate::CodeMode;
use crate::Ctx;
use crate::CtxVal;
use crate::FuncMode;
use crate::Map;
use crate::Mode;
use crate::Pair;
use crate::Symbol;
use crate::SymbolMode;
use crate::Val;
use crate::ctx::map::CtxGuard;
use crate::ctx::map::CtxMap;
use crate::ctx::map::CtxValue;
use crate::utils::val::map_remove;
use crate::utils::val::symbol;

const CONST: &str = "constant";
const STATIC: &str = "static";
const LOCK: &str = "lock";

pub(crate) const VARIABLES: &str = "variables";
pub(crate) const REVERSE: &str = "reverse";

pub(crate) fn parse_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        FuncMode::map_mode(
            Map::default(),
            FuncMode::prim_mode(SymbolMode::Literal, CodeMode::Form),
            FuncMode::default_mode(),
        ),
    );
    FuncMode::map_mode(map, FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
}

pub(crate) fn parse_ctx(input: Val) -> Option<CtxVal> {
    let Val::Map(mut map) = input else {
        return None;
    };
    let variables = match map_remove(&mut map, VARIABLES) {
        Val::Unit(_) => Map::default(),
        Val::Map(map) => Map::from(map),
        _ => return None,
    };
    let variables = parse_variables(variables)?;
    let reverse = match map_remove(&mut map, REVERSE) {
        Val::Unit(_) => false,
        Val::Bit(b) => b.bool(),
        _ => return None,
    };
    let variables = CtxMap::new(variables, reverse);
    let ctx = Ctx::new(variables);
    Some(ctx.into())
}

fn parse_variables(map: Map<Val, Val>) -> Option<Map<Symbol, CtxValue>> {
    map.into_iter()
        .map(|(binding, val)| {
            let binding = parse_binding(binding)?;
            let ctx_value = CtxValue::new(val, binding.guard);
            Some((binding.name, ctx_value))
        })
        .collect()
}

fn parse_binding(val: Val) -> Option<Binding> {
    match val {
        Val::Symbol(name) => Some(Binding { name, guard: CtxGuard::default() }),
        Val::Call(call) => {
            if call.reverse || !call.func.is_unit() {
                return None;
            }
            let call = Call::from(call);
            let Val::Pair(pair) = call.input else {
                return None;
            };
            let pair = Pair::from(pair);
            let Val::Symbol(name) = pair.first else {
                return None;
            };
            let guard = parse_guard(pair.second, CtxGuard::default())?;
            Some(Binding { name, guard })
        }
        _ => None,
    }
}

pub(crate) fn parse_guard(guard: Val, mut default: CtxGuard) -> Option<CtxGuard> {
    match guard {
        Val::Symbol(s) => match &*s {
            CONST => default.const1 = true,
            STATIC => default.static1 = true,
            LOCK => default.lock = true,
            _ => return None,
        },
        Val::Map(mut map) => {
            default.const1 = parse_bool(&mut map, CONST)?;
            default.static1 = parse_bool(&mut map, STATIC)?;
            default.lock = parse_bool(&mut map, LOCK)?;
        }
        _ => return None,
    }
    Some(default)
}

fn parse_bool(map: &mut Map<Val, Val>, key: &str) -> Option<bool> {
    let b = match map.remove(&symbol(key)) {
        Some(Val::Unit(_)) => true,
        Some(Val::Bit(bit)) => bit.bool(),
        None => false,
        _ => return None,
    };
    Some(b)
}

pub(crate) fn generate_ctx(ctx: CtxVal) -> Val {
    let ctx = Ctx::from(ctx).destruct();
    let mut map = Map::default();
    let reverse = ctx.variables.is_reverse();
    if reverse {
        map.insert(symbol(REVERSE), Val::Bit(Bit::true1()));
    }
    if let Some(variables) = generate_variables(ctx.variables) {
        map.insert(symbol(VARIABLES), variables);
    }
    Val::Map(map.into())
}

fn generate_variables(ctx_map: CtxMap) -> Option<Val> {
    if ctx_map.is_empty() {
        return None;
    }
    let map: Map<Val, Val> = ctx_map
        .unwrap()
        .into_iter()
        .map(|(name, v)| {
            let k = generate_binding(Binding { name, guard: v.guard });
            let v = v.val;
            (k, v)
        })
        .collect();
    Some(Val::Map(map.into()))
}

fn generate_binding(binding: Binding) -> Val {
    if binding.guard == CtxGuard::default() {
        return Val::Symbol(binding.name);
    }
    let guard = generate_guard(binding.guard);
    let pair = Pair::new(Val::Symbol(binding.name), guard);
    Val::Call(Call::new(false, Val::default(), Val::Pair(pair.into())).into())
}

fn generate_guard(guard: CtxGuard) -> Val {
    if guard.const1 && !guard.static1 && !guard.lock {
        return symbol(CONST);
    }
    if guard.static1 && !guard.const1 && !guard.lock {
        return symbol(STATIC);
    }
    if guard.lock && !guard.static1 && !guard.const1 {
        return symbol(LOCK);
    }
    let mut map = Map::default();
    if guard.static1 {
        map.insert(symbol(STATIC), Val::default());
    }
    if guard.const1 {
        map.insert(symbol(CONST), Val::default());
    }
    if guard.lock {
        map.insert(symbol(LOCK), Val::default());
    }
    Val::Map(map.into())
}

pub(crate) struct Binding {
    pub(crate) name: Symbol,
    pub(crate) guard: CtxGuard,
}
