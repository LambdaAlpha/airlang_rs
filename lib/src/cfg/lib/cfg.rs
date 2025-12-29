use std::collections::HashMap;

use log::error;

use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::dyn_impl;
use crate::cfg::lib::free_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::cfg::StepsExceed;
use crate::semantics::core::Eval;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::DynRef;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::utils::guard::guard;

// todo design more
#[derive(Clone)]
pub struct CfgLib {
    pub new: FreePrimFuncVal,
    pub represent: FreePrimFuncVal,
    pub length: FreePrimFuncVal,
    pub snapshot: FreePrimFuncVal,
    pub exist: FreePrimFuncVal,
    pub import: FreePrimFuncVal,
    pub export: FreePrimFuncVal,
    pub with: MutPrimFuncVal,
    pub where_: MutPrimFuncVal,
}

impl Default for CfgLib {
    fn default() -> Self {
        CfgLib {
            new: new(),
            represent: represent(),
            length: length(),
            snapshot: snapshot(),
            exist: exist(),
            import: import(),
            export: export(),
            with: with(),
            where_: where_(),
        }
    }
}

impl CfgMod for CfgLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_config.new", self.new);
        extend_func(cfg, "_config.represent", self.represent);
        extend_func(cfg, "_config.length", self.length);
        extend_func(cfg, "_config.snapshot", self.snapshot);
        extend_func(cfg, "_config.exist", self.exist);
        extend_func(cfg, "_config.import", self.import);
        extend_func(cfg, "_config.export", self.export);
        extend_func(cfg, "_config.with", self.with);
        extend_func(cfg, "_config.where", self.where_);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let mut new_cfg = Cfg::default();
    let list = List::from(list);
    for val in list {
        let Val::Map(map) = val else {
            error!("list.item {val:?} should be a map");
            return illegal_input(cfg);
        };
        let map = Map::from(map);
        for (k, v) in map {
            new_cfg.extend_scope(k, v);
        }
        new_cfg.begin_scope();
    }
    new_cfg.end_scope();
    Val::Cfg(new_cfg.into())
}

pub fn represent() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_represent) }.free()
}

fn fn_represent(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Cfg(new_cfg) = input else {
        error!("input {input:?} should be a cfg");
        return illegal_input(cfg);
    };
    let new_cfg = Cfg::from(new_cfg);
    let scope_level = new_cfg.scope_level();
    let mut map_scopes = HashMap::new();
    for (name, scopes) in new_cfg {
        for (scope, val) in scopes {
            let map = map_scopes.entry(scope).or_insert_with(Map::default);
            map.insert(name.clone(), val);
        }
    }
    let mut list = List::default();
    for i in 0 ..= scope_level {
        let map = map_scopes.remove(&i).unwrap_or_default();
        list.push(Val::Map(map.into()));
    }
    Val::List(list.into())
}

pub fn length() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_length) }.free()
}

fn fn_length(cfg: &mut Cfg, _input: Val) -> Val {
    Val::Int(Int::from(cfg.len()).into())
}

pub fn snapshot() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_snapshot) }.free()
}

fn fn_snapshot(cfg: &mut Cfg, _input: Val) -> Val {
    Val::Cfg(cfg.snapshot().into())
}

pub fn exist() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_exist) }.free()
}

fn fn_exist(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    let exist = cfg.exist(name);
    Val::Bit(Bit::from(exist))
}

pub fn import() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_import) }.free()
}

fn fn_import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    cfg.import(name).unwrap_or_default()
}

pub fn export() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_export) }.free()
}

fn fn_export(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Key(name) = pair.first else {
        error!("input.first {:?} should be a key", pair.first);
        return illegal_input(cfg);
    };
    cfg.export(name, pair.second);
    Val::default()
}

pub fn with() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: dyn_impl(fn_with) }.mut_()
}

fn fn_with(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    cfg.begin_scope();
    guard(
        cfg,
        |cfg| {
            let Val::Pair(pair) = input else {
                error!("input {input:?} should be a pair");
                return illegal_input(cfg);
            };
            let pair = Pair::from(pair);
            let Val::Map(map) = pair.first else {
                error!("input.first {:?} should be a map", pair.first);
                return illegal_input(cfg);
            };
            let map = Map::from(map);
            for (k, v) in map {
                cfg.extend_scope(k, v);
            }
            Eval.dyn_call(&mut **cfg, ctx, pair.second)
        },
        Cfg::end_scope,
    )
}

pub fn where_() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: dyn_impl(fn_where) }.mut_()
}

fn fn_where(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Cfg(new_cfg) = pair.first else {
        error!("input.first {:?} should be a cfg", pair.first);
        return illegal_input(cfg);
    };
    let mut new_cfg = Cfg::from(new_cfg);
    StepsExceed::catch(|| Eval.dyn_call(&mut new_cfg, ctx, pair.second)).unwrap_or_default()
}
