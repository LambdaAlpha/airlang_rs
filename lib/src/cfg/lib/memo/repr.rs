use const_format::concatcp;
use log::error;

use crate::cfg::utils::symbol;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::memo::Contract;
use crate::semantics::memo::Memo;
use crate::semantics::memo::MemoMap;
use crate::semantics::memo::MemoValue;
use crate::semantics::val::MemoVal;
use crate::semantics::val::Val;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;

const NONE: &str = concatcp!(PREFIX_ID, "none");
const STILL: &str = concatcp!(PREFIX_ID, "still");
const FINAL: &str = concatcp!(PREFIX_ID, "final");
const STATIC: &str = concatcp!(PREFIX_ID, "static");
const CONST: &str = concatcp!(PREFIX_ID, "constant");

pub(in crate::cfg) fn parse_memo(input: Val) -> Option<MemoVal> {
    let Val::Map(map) = input else {
        error!("input {input:?} should be a map");
        return None;
    };
    let variables = parse_variables(Map::from(map))?;
    let variables = MemoMap::new(variables);
    let memo = Memo::new(variables);
    Some(memo.into())
}

fn parse_variables(map: Map<Val, Val>) -> Option<Map<Symbol, MemoValue>> {
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
            Some((name, MemoValue::new(pair.second, contract)))
        })
        .collect()
}

pub(in crate::cfg) fn parse_contract(contract: &Val) -> Option<Contract> {
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

pub(in crate::cfg) fn generate_memo(memo: MemoVal) -> Val {
    let map = Memo::from(memo).unwrap();
    generate_variables(map)
}

fn generate_variables(memo_map: MemoMap) -> Val {
    let map: Map<Val, Val> = memo_map
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

pub(in crate::cfg) fn generate_contract(contract: Contract) -> Val {
    let s = match contract {
        Contract::None => NONE,
        Contract::Still => STILL,
        Contract::Final => FINAL,
        Contract::Static => STATIC,
        Contract::Const => CONST,
    };
    symbol(s)
}
