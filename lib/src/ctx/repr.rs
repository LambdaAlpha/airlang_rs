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
use crate::ctx::map::Contract;
use crate::ctx::map::CtxMap;
use crate::ctx::map::CtxValue;
use crate::utils::val::map_remove;
use crate::utils::val::symbol;

const NONE: &str = "none";
const STATIC: &str = "static";
const STILL: &str = "still";
const FINAL: &str = "final";
const CONST: &str = "constant";

pub(crate) const VARIABLES: &str = "variables";

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

// todo design
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
    let variables = CtxMap::new(variables);
    let ctx = Ctx::new(variables);
    Some(ctx.into())
}

fn parse_variables(map: Map<Val, Val>) -> Option<Map<Symbol, CtxValue>> {
    map.into_iter()
        .map(|(binding, val)| {
            let binding = parse_binding(binding)?;
            let ctx_value = CtxValue::new(val, binding.contract.unwrap_or_default());
            Some((binding.name, ctx_value))
        })
        .collect()
}

// todo design
fn parse_binding(val: Val) -> Option<OptBinding> {
    match val {
        Val::Symbol(name) => Some(OptBinding { name, contract: None }),
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
            let contract = parse_contract(pair.second)?;
            Some(OptBinding { name, contract: Some(contract) })
        }
        _ => None,
    }
}

pub(crate) fn parse_contract(contract: Val) -> Option<Contract> {
    let Val::Symbol(s) = contract else {
        return None;
    };
    let contract = match &*s {
        NONE => Contract::None,
        STATIC => Contract::Static,
        STILL => Contract::Still,
        FINAL => Contract::Final,
        CONST => Contract::Const,
        _ => return None,
    };
    Some(contract)
}

// todo design
pub(crate) fn generate_ctx(ctx: CtxVal) -> Val {
    let ctx = Ctx::from(ctx).destruct();
    let mut map = Map::default();
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
            let k = generate_binding(Binding { name, contract: v.contract });
            let v = v.val;
            (k, v)
        })
        .collect();
    Some(Val::Map(map.into()))
}

// todo design
fn generate_binding(binding: Binding) -> Val {
    if binding.contract == Contract::default() {
        return Val::Symbol(binding.name);
    }
    let contract = generate_contract(binding.contract);
    let pair = Pair::new(Val::Symbol(binding.name), contract);
    Val::Call(Call::new(false, Val::default(), Val::Pair(pair.into())).into())
}

pub(crate) fn generate_contract(contract: Contract) -> Val {
    let s = match contract {
        Contract::None => NONE,
        Contract::Static => STATIC,
        Contract::Still => STILL,
        Contract::Final => FINAL,
        Contract::Const => CONST,
    };
    symbol(s)
}

pub(crate) struct Binding {
    pub(crate) name: Symbol,
    pub(crate) contract: Contract,
}

pub(crate) struct OptBinding {
    pub(crate) name: Symbol,
    pub(crate) contract: Option<Contract>,
}
