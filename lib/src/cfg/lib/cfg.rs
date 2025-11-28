use std::collections::HashMap;

use log::error;

use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_input;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::free_impl;
use crate::cfg::lib::mut_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::cfg::StepsExceed;
use crate::semantics::core::Eval;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::utils::gurad::guard;

// todo design more
#[derive(Clone)]
pub struct CfgLib {
    pub new: FreePrimFuncVal,
    pub repr: FreePrimFuncVal,
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
            repr: repr(),
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
        self.new.extend(cfg);
        self.repr.extend(cfg);
        self.length.extend(cfg);
        self.snapshot.extend(cfg);
        self.exist.extend(cfg);
        self.import.extend(cfg);
        self.export.extend(cfg);
        self.with.extend(cfg);
        self.where_.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "_config.new", raw_input: false, f: free_impl(fn_new) }.free()
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

pub fn repr() -> FreePrimFuncVal {
    FreePrimFn { id: "_config.represent", raw_input: false, f: free_impl(fn_repr) }.free()
}

fn fn_repr(cfg: &mut Cfg, input: Val) -> Val {
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
    FreePrimFn { id: "_config.length", raw_input: false, f: free_impl(fn_length) }.free()
}

fn fn_length(cfg: &mut Cfg, _input: Val) -> Val {
    Val::Int(Int::from(cfg.len()).into())
}

pub fn snapshot() -> FreePrimFuncVal {
    FreePrimFn { id: "_config.snapshot", raw_input: false, f: free_impl(fn_snapshot) }.free()
}

fn fn_snapshot(cfg: &mut Cfg, _input: Val) -> Val {
    Val::Cfg(cfg.snapshot().into())
}

pub fn exist() -> FreePrimFuncVal {
    FreePrimFn { id: "_config.exist", raw_input: false, f: free_impl(fn_exist) }.free()
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
    FreePrimFn { id: "_config.import", raw_input: false, f: free_impl(fn_import) }.free()
}

fn fn_import(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(name) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    cfg.import(name).unwrap_or_default()
}

pub fn export() -> FreePrimFuncVal {
    FreePrimFn { id: "_config.export", raw_input: false, f: free_impl(fn_export) }.free()
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
    DynPrimFn { id: "_config.with", raw_input: false, f: mut_impl(fn_with) }.mut_()
}

fn fn_with(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
            Eval.mut_call(&mut **cfg, ctx, pair.second)
        },
        Cfg::end_scope,
    )
}

pub fn where_() -> MutPrimFuncVal {
    DynPrimFn { id: "_config.where", raw_input: false, f: mut_impl(fn_where) }.mut_()
}

fn fn_where(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
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
    StepsExceed::catch(|| Eval.mut_call(&mut new_cfg, ctx, pair.second)).unwrap_or_default()
}
