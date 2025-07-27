use log::error;

use crate::prelude::mode::FuncMode;
use crate::prelude::mode::Mode;
use crate::prelude::mode::SymbolMode;
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

pub(super) fn parse_mode() -> Option<Mode> {
    FuncMode::map_mode(
        Map::default(),
        FuncMode::pair_mode(
            Map::default(),
            FuncMode::symbol_mode(SymbolMode::Literal),
            FuncMode::default_mode(),
        ),
    )
}

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
            let Val::Pair(pair) = val else {
                return None;
            };
            let pair = Pair::from(pair);
            let contract = parse_contract(&pair.first)?;
            Some((name, CtxValue::new(pair.second, contract)))
        })
        .collect()
}

pub(in crate::prelude) fn parse_contract(contract: &Val) -> Option<Contract> {
    match contract {
        Val::Unit(_) => Some(Contract::None),
        Val::Symbol(s) => {
            let contract = match &**s {
                NONE => Contract::None,
                STILL => Contract::Still,
                FINAL => Contract::Final,
                STATIC => Contract::Static,
                CONST => Contract::Const,
                s => {
                    error!(
                        "contract {s:?} should be one of {NONE}, {STILL}, {FINAL}, {STATIC} or {CONST}"
                    );
                    return None;
                }
            };
            Some(contract)
        }
        contract => {
            error!("contract {contract:?} should be a symbol");
            None
        }
    }
}

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
