use crate::{
    semantics::{
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        prelude::names,
        val::MapVal,
        Val,
    },
    types::{
        Pair,
        Symbol,
    },
};

pub(crate) fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or_default()
}

const EVAL_MODE: &str = "eval_mode";
const PAIR_EVAL_MODE: &str = "pair_eval_mode";

pub(crate) fn parse_eval_mode(map: &mut MapVal) -> Option<EvalMode> {
    let eval_mode = map_remove(map, EVAL_MODE);
    let default = if let Val::Unit(_) = eval_mode {
        BasicEvalMode::Eval
    } else if let Some(eval_mode) = parse_basic_eval_mode(eval_mode) {
        eval_mode
    } else {
        return None;
    };
    let pair_eval_mode = map_remove(map, PAIR_EVAL_MODE);
    let pair = match pair_eval_mode {
        Val::Pair(pair) => {
            let first = parse_basic_eval_mode(pair.first)?;
            let second = parse_basic_eval_mode(pair.second)?;
            Some((first, second))
        }
        Val::Unit(_) => None,
        _ => return None,
    };
    Some(EvalMode { default, pair })
}

fn parse_basic_eval_mode(val: Val) -> Option<BasicEvalMode> {
    let Val::Symbol(name) = val else {
        return None;
    };
    let eval_mode = match &*name {
        names::VALUE => BasicEvalMode::Value,
        names::EVAL => BasicEvalMode::Eval,
        names::QUOTE => BasicEvalMode::Quote,
        names::INLINE => BasicEvalMode::Inline,
        _ => return None,
    };
    Some(eval_mode)
}

pub(crate) fn basic_eval_mode_to_symbol(eval_mode: BasicEvalMode) -> Symbol {
    let str = match eval_mode {
        BasicEvalMode::Value => names::VALUE,
        BasicEvalMode::Eval => names::EVAL,
        BasicEvalMode::Quote => names::QUOTE,
        BasicEvalMode::Inline => names::INLINE,
    };
    Symbol::from_str(str)
}

pub(crate) fn generate_eval_mode(map: &mut MapVal, eval_mode: EvalMode) {
    if eval_mode.default != BasicEvalMode::Eval {
        map.insert(
            symbol(EVAL_MODE),
            Val::Symbol(basic_eval_mode_to_symbol(eval_mode.default)),
        );
    }
    if let Some((first, second)) = eval_mode.pair {
        let first = Val::Symbol(basic_eval_mode_to_symbol(first));
        let second = Val::Symbol(basic_eval_mode_to_symbol(second));
        let pair = Val::Pair(Box::new(Pair::new(first, second)));
        map.insert(symbol(PAIR_EVAL_MODE), pair);
    }
}

pub(crate) fn symbol(s: &str) -> Val {
    Val::Symbol(Symbol::from_str(s))
}
