use std::collections::HashMap;

use log::error;

use crate::cfg::CfgMod;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::Library;
use crate::cfg::lib::free_impl;
use crate::cfg::lib::memo_put_func;
use crate::cfg::lib::mut_impl;
use crate::cfg::mode::FuncMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::MutFn;
use crate::semantics::memo::Memo;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CfgLib {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
    pub import: FreePrimFuncVal,
    pub export: FreePrimFuncVal,
    pub with: MutPrimFuncVal,
}

impl Default for CfgLib {
    fn default() -> Self {
        CfgLib { new: new(), repr: repr(), import: import(), export: export(), with: with() }
    }
}

impl CfgMod for CfgLib {
    fn extend(self, cfg: &Cfg) {
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.import.extend(cfg);
        self.export.extend(cfg);
        self.with.extend(cfg);
    }
}

impl Library for CfgLib {
    fn prelude(&self, memo: &mut Memo) {
        memo_put_func(memo, "import", &self.import);
        memo_put_func(memo, "export", &self.export);
        memo_put_func(memo, "with", &self.with);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "configuration.new", f: free_impl(fn_new), mode: FuncMode::default_mode() }
        .free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
    };
    let mut cfg = Cfg::default();
    let list = List::from(list);
    for val in list {
        let Val::Map(map) = val else {
            error!("list.item {val:?} should be a map");
            return Val::default();
        };
        let map = Map::from(map);
        for (k, v) in map {
            let Val::Symbol(name) = k else {
                error!("list.item.key {k:?} should be a symbol");
                return Val::default();
            };
            cfg.extend_scope(name, v);
        }
        cfg.begin_scope();
    }
    cfg.end_scope();
    Val::Cfg(cfg.into())
}

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn {
        id: "configuration.represent",
        f: free_impl(fn_repr),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_repr(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Cfg(cfg) = input else {
        error!("input {input:?} should be a cfg");
        return Val::default();
    };
    let cfg = Cfg::from(cfg);
    let scope_level = cfg.scope_level();
    let mut map_scopes = HashMap::new();
    for (name, scopes) in cfg {
        for (scope, val) in scopes {
            let map = map_scopes.entry(scope).or_insert_with(Map::default);
            map.insert(Val::Symbol(name.clone()), val);
        }
    }
    let mut list = List::default();
    for i in 0 ..= scope_level {
        let map = map_scopes.remove(&i).unwrap_or_default();
        list.push(Val::Map(map.into()));
    }
    Val::List(list.into())
}

pub fn import() -> FreePrimFuncVal {
    FreePrimFn {
        id: "configuration.import",
        f: free_impl(fn_import),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Symbol(name) = input else {
        error!("input {input:?} should be a symbol");
        return Val::default();
    };
    cfg.import(name).unwrap_or_default()
}

pub fn export() -> FreePrimFuncVal {
    FreePrimFn {
        id: "configuration.export",
        f: free_impl(fn_export),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_export(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(name) = pair.first else {
        error!("input.first {:?} should be a symbol", pair.first);
        return Val::default();
    };
    cfg.export(name, pair.second);
    Val::default()
}

pub fn with() -> MutPrimFuncVal {
    DynPrimFn {
        id: "configuration.with",
        f: mut_impl(fn_with),
        mode: FuncMode::pair_mode(Map::default(), FuncMode::default_mode(), FuncMode::id_mode()),
    }
    .mut_()
}

fn fn_with(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    cfg.begin_scope();
    let output = 'scope: {
        let Val::Pair(pair) = input else {
            error!("input {input:?} should be a pair");
            break 'scope Val::default();
        };
        let pair = Pair::from(pair);
        let Val::Map(map) = pair.first else {
            error!("input.first {:?} should be a map", pair.first);
            break 'scope Val::default();
        };
        let map = Map::from(map);
        for (k, v) in map {
            let Val::Symbol(name) = k else {
                error!("input.first.key {k:?} should be a symbol");
                break 'scope Val::default();
            };
            cfg.extend_scope(name, v);
        }
        Eval.mut_call(cfg, ctx, pair.second)
    };
    cfg.end_scope();
    output
}
