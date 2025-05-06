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
use crate::ctx::map::CtxMap;
use crate::ctx::map::CtxMapRef;
use crate::ctx::map::CtxValue;
use crate::ctx::map::VarAccess;
use crate::utils::val::map_remove;
use crate::utils::val::symbol;

const ACCESS: &str = "access";
const STATIC: &str = "static";

pub(crate) const ASSIGNABLE: &str = "assignable";
pub(crate) const MUTABLE: &str = "mutable";
pub(crate) const CONST: &str = "constant";

pub(crate) const VARIABLES: &str = "variables";
pub(crate) const REVERSE: &str = "reverse";
pub(crate) const SOLVER: &str = "solver";

pub(crate) fn parse_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        FuncMode::map_mode(
            Map::default(),
            FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal),
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
    let solver = match map_remove(&mut map, SOLVER) {
        Val::Unit(_) => None,
        Val::Func(solver) => Some(solver),
        _ => return None,
    };
    let ctx = Ctx::new(variables, solver);
    Some(ctx.into())
}

fn parse_variables(map: Map<Val, Val>) -> Option<Map<Symbol, CtxValue>> {
    map.into_iter()
        .map(|(binding, val)| {
            let binding = parse_binding(binding)?;
            let ctx_value =
                CtxValue { access: binding.extra.access, static1: binding.extra.static1, val };
            Some((binding.name, ctx_value))
        })
        .collect()
}

fn parse_binding(val: Val) -> Option<Binding> {
    match val {
        Val::Symbol(name) => Some(Binding { name, extra: Extra::default() }),
        Val::Call(call) => {
            if !call.func.is_unit() {
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
            let extra = parse_extra(pair.second, Extra::default())?;
            Some(Binding { name, extra })
        }
        _ => None,
    }
}

pub(crate) fn parse_extra(extra: Val, mut default: Extra) -> Option<Extra> {
    match extra {
        Val::Symbol(s) => match &*s {
            ASSIGNABLE => default.access = VarAccess::Assign,
            MUTABLE => default.access = VarAccess::Mut,
            CONST => default.access = VarAccess::Const,
            STATIC => default.static1 = true,
            _ => return None,
        },
        Val::Map(mut map) => {
            match map_remove(&mut map, ACCESS) {
                Val::Symbol(access) => {
                    default.access = parse_var_access(&access)?;
                }
                Val::Unit(_) => {}
                _ => return None,
            }
            let static1 = match map.remove(&symbol(STATIC)) {
                Some(Val::Unit(_)) => true,
                Some(Val::Bit(bit)) => bit.bool(),
                None => false,
                _ => return None,
            };
            default.static1 = static1;
        }
        _ => return None,
    }
    Some(default)
}

pub(crate) fn parse_var_access(access: &str) -> Option<VarAccess> {
    let access = match access {
        ASSIGNABLE => VarAccess::Assign,
        MUTABLE => VarAccess::Mut,
        CONST => VarAccess::Const,
        _ => return None,
    };
    Some(access)
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
    if let Some(solver) = ctx.solver {
        map.insert(symbol(SOLVER), Val::Func(solver));
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
            let extra = Extra { access: v.access, static1: v.static1 };
            let k = generate_binding(Binding { name, extra });
            let v = v.val;
            (k, v)
        })
        .collect();
    Some(Val::Map(map.into()))
}

fn generate_binding(binding: Binding) -> Val {
    if binding.extra == Extra::default() {
        return Val::Symbol(binding.name);
    }
    let extra = generate_extra(binding.extra);
    let pair = Pair::new(Val::Symbol(binding.name), extra);
    Val::Call(Call::new(Val::default(), Val::Pair(pair.into())).into())
}

fn generate_extra(extra: Extra) -> Val {
    if !extra.static1 {
        return Val::Symbol(generate_var_access(extra.access));
    }
    if extra.access == VarAccess::default() {
        return symbol(STATIC);
    }
    let mut map = Map::default();
    map.insert(symbol(STATIC), Val::default());
    map.insert(symbol(ACCESS), Val::Symbol(generate_var_access(extra.access)));
    Val::Map(map.into())
}

pub(crate) fn generate_var_access(access: VarAccess) -> Symbol {
    let access = match access {
        VarAccess::Assign => ASSIGNABLE,
        VarAccess::Mut => MUTABLE,
        VarAccess::Const => CONST,
    };
    Symbol::from_str(access)
}

pub(crate) struct Binding {
    pub(crate) name: Symbol,
    pub(crate) extra: Extra,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Extra {
    pub(crate) access: VarAccess,
    pub(crate) static1: bool,
}
