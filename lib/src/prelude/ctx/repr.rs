use log::error;

use crate::prelude::mode::FuncMode;
use crate::prelude::mode::Mode;
use crate::prelude::utils::symbol;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxMap;
use crate::semantics::ctx::CtxValue;
use crate::semantics::val::CtxVal;
use crate::semantics::val::Val;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;

const NONE: &str = "none";
const STILL: &str = "still";
const FINAL: &str = "final";
const STATIC: &str = "static";
const CONST: &str = "constant";

pub(in crate::prelude) struct OptBinding {
    pub(super) name: Symbol,
    pub(super) contract: Option<Contract>,
}

pub(super) fn parse_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(symbol(NONE), FuncMode::default_mode());
    map.insert(symbol(STILL), FuncMode::default_mode());
    map.insert(symbol(FINAL), FuncMode::default_mode());
    map.insert(symbol(STATIC), FuncMode::default_mode());
    map.insert(symbol(CONST), FuncMode::default_mode());
    FuncMode::map_mode(
        Map::default(),
        FuncMode::pair_mode(map, FuncMode::default_mode(), FuncMode::default_mode()),
    )
}

// todo design
pub(super) fn parse_ctx(input: Val) -> Option<CtxVal> {
    let Val::Map(map) = input else {
        error!("input {input:?} should be a map");
        return None;
    };
    let variables = parse_variables(Map::from(map))?;
    let variables = CtxMap::new(variables);
    let ctx = Ctx::new(variables);
    Some(ctx.into())
}

fn parse_variables(map: Map<Val, Val>) -> Option<Map<Symbol, CtxValue>> {
    map.into_iter()
        .map(|(name, val)| {
            let Val::Symbol(name) = name else {
                return None;
            };
            // todo design
            let Val::Pair(pair) = val else {
                return Some((name, CtxValue::new(val, Contract::None)));
            };
            let pair = Pair::from(pair);
            if let Some(contract) = parse_contract(&pair.first) {
                Some((name, CtxValue::new(pair.second, contract)))
            } else {
                Some((name, CtxValue::new(Val::Pair(pair.into()), Contract::None)))
            }
        })
        .collect()
}

pub(in crate::prelude) fn parse_contract(contract: &Val) -> Option<Contract> {
    let Val::Symbol(s) = contract else {
        error!("contract {contract:?} should be a symbol");
        return None;
    };
    let contract = match &**s {
        NONE => Contract::None,
        STILL => Contract::Still,
        FINAL => Contract::Final,
        STATIC => Contract::Static,
        CONST => Contract::Const,
        s => {
            error!("contract {s:?} should be one of {NONE}, {STILL}, {FINAL}, {STATIC} or {CONST}");
            return None;
        }
    };
    Some(contract)
}

// todo design
pub(super) fn generate_ctx(ctx: CtxVal) -> Val {
    let ctx = Ctx::from(ctx).destruct();
    generate_variables(ctx.variables)
}

fn generate_variables(ctx_map: CtxMap) -> Val {
    let map: Map<Val, Val> = ctx_map
        .unwrap()
        .into_iter()
        .map(|(name, v)| {
            let contract = generate_contract(v.contract);
            let pair = Val::Pair(Pair::new(contract, v.val).into());
            (Val::Symbol(name), pair)
        })
        .collect();
    Val::Map(map.into())
}

pub(in crate::prelude) fn generate_contract(contract: Contract) -> Val {
    let s = match contract {
        Contract::None => NONE,
        Contract::Still => STILL,
        Contract::Final => FINAL,
        Contract::Static => STATIC,
        Contract::Const => CONST,
    };
    symbol(s)
}
