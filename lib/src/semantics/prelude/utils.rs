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
    types::Symbol,
};

pub(crate) fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or_default()
}

pub(crate) fn parse_eval_mode(map: &mut MapVal) -> Option<EvalMode> {
    let eval_mode = map_remove(map, "eval_mode");
    let default = if let Val::Unit(_) = eval_mode {
        BasicEvalMode::Eval
    } else if let Some(eval_mode) = parse_basic_eval_mode(eval_mode) {
        eval_mode
    } else {
        return None;
    };
    let pair_eval_mode = map_remove(map, "pair_eval_mode");
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
        names::EVAL_QUOTE => BasicEvalMode::Quote,
        names::EVAL_INLINE => BasicEvalMode::Inline,
        _ => return None,
    };
    Some(eval_mode)
}

pub(crate) fn basic_eval_mode_to_symbol(eval_mode: BasicEvalMode) -> Symbol {
    let str = match eval_mode {
        BasicEvalMode::Value => "value",
        BasicEvalMode::Eval => "eval",
        BasicEvalMode::Quote => "quote",
        BasicEvalMode::Inline => "inline",
    };
    Symbol::from_str(str)
}
