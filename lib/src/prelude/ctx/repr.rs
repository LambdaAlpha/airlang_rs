use log::error;

use crate::prelude::mode::CodeMode;
use crate::prelude::mode::FuncMode;
use crate::prelude::mode::Mode;
use crate::prelude::mode::SymbolMode;
use crate::prelude::utils::map_remove;
use crate::prelude::utils::symbol;
use crate::semantics::ctx::Contract;
use crate::semantics::ctx::Ctx;
use crate::semantics::ctx::CtxMap;
use crate::semantics::ctx::CtxValue;
use crate::semantics::val::CtxVal;
use crate::semantics::val::Val;
use crate::type_::Action;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;

const NONE: &str = "none";
const STATIC: &str = "static";
const STILL: &str = "still";
const FINAL: &str = "final";
const CONST: &str = "constant";

const VARIABLES: &str = "variables";

pub(super) fn parse_mode() -> Option<Mode> {
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
pub(super) fn parse_ctx(input: Val) -> Option<CtxVal> {
    let Val::Map(mut map) = input else {
        error!("input {input:?} should be a map");
        return None;
    };
    let variables = match map_remove(&mut map, VARIABLES) {
        Val::Unit(_) => Map::default(),
        Val::Map(map) => Map::from(map),
        v => {
            error!("variables {v:?} should be a map or a unit");
            return None;
        }
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
        Val::Task(task) => {
            if task.action != Action::Call || !task.func.is_unit() {
                error!("task {task:?} should be call and task.func should be a unit");
                return None;
            }
            let task = Task::from(task);
            let Val::Pair(pair) = task.input else {
                error!("task.input {:?} should be a pair", task.input);
                return None;
            };
            let pair = Pair::from(pair);
            let Val::Symbol(name) = pair.first else {
                error!("name {:?} should be a symbol", pair.first);
                return None;
            };
            let contract = parse_contract(pair.second)?;
            Some(OptBinding { name, contract: Some(contract) })
        }
        _ => None,
    }
}

pub(in crate::prelude) fn parse_contract(contract: Val) -> Option<Contract> {
    let Val::Symbol(s) = contract else {
        error!("contract {contract:?} should be a symbol");
        return None;
    };
    let contract = match &*s {
        NONE => Contract::None,
        STATIC => Contract::Static,
        STILL => Contract::Still,
        FINAL => Contract::Final,
        CONST => Contract::Const,
        s => {
            error!("contract {s:?} should be one of {NONE}, {STATIC}, {STILL}, {FINAL} or {CONST}");
            return None;
        }
    };
    Some(contract)
}

// todo design
pub(super) fn generate_ctx(ctx: CtxVal) -> Val {
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
    let task = Task {
        action: Action::Call,
        func: Val::default(),
        ctx: Val::default(),
        input: Val::Pair(pair.into()),
    };
    Val::Task(task.into())
}

pub(in crate::prelude) fn generate_contract(contract: Contract) -> Val {
    let s = match contract {
        Contract::None => NONE,
        Contract::Static => STATIC,
        Contract::Still => STILL,
        Contract::Final => FINAL,
        Contract::Const => CONST,
    };
    symbol(s)
}

pub(in crate::prelude) struct Binding {
    pub(super) name: Symbol,
    pub(super) contract: Contract,
}

pub(in crate::prelude) struct OptBinding {
    pub(super) name: Symbol,
    pub(super) contract: Option<Contract>,
}
