use std::collections::hash_map::Entry;

use const_format::concatcp;

use crate::semantics::core::PREFIX_CELL;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;

pub trait CfgMod {
    fn export(self, cfg: &mut Map<Key, Val>);
}

pub fn import<'a>(cfg: &'a Map<Key, Val>, key: &'static str) -> Option<&'a Val> {
    cfg.get(&Key::from_str_unchecked(key))
}

pub fn export(cfg: &mut Map<Key, Val>, key: &'static str, val: impl Into<Val>) {
    match cfg.entry(Key::from_str_unchecked(key)) {
        Entry::Occupied(_) => panic!("expect a unique key, but {key} is already used"),
        Entry::Vacant(entry) => {
            entry.insert(val.into());
        },
    }
}

pub fn export_func(cfg: &mut Map<Key, Val>, key: &'static str, val: PrimFuncVal) {
    export(cfg, key, Val::Func(val.into()));
}

pub const PRELUDE: &str = concatcp!(PREFIX_CELL, "prelude");

pub fn prelude(cfg: &Map<Key, Val>) -> Val {
    let prelude = import(cfg, PRELUDE);
    let Some(prelude) = prelude else {
        panic!("value not found for key {PRELUDE} in config");
    };
    let Val::Link(prelude) = prelude else {
        panic!("expected {PRELUDE} to be a link, but got {prelude}");
    };
    let prelude = prelude.clone();
    let Ok(prelude) = prelude.try_borrow() else {
        panic!("link is not available");
    };
    prelude.clone()
}

pub mod prim;

pub mod comp;

pub mod error;

mod repr;

mod ctx;

mod utils;
