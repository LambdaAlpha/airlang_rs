use const_format::concatcp;
use num_traits::ToPrimitive;

use crate::bug;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::CALL;
use crate::semantics::val::CELL;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::IntVal;
use crate::semantics::val::LIST;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PAIR;
use crate::semantics::val::PairVal;
use crate::semantics::val::QUOTE;
use crate::semantics::val::QuoteVal;
use crate::semantics::val::Val;
use crate::type_::Key;

pub(crate) const CELL_VALUE: &str = concatcp!(PREFIX_CELL, CELL, ".value");

impl DynCtx<Key, Val> for CellVal {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Val> {
        if &*key == CELL_VALUE {
            return Some(&self.value);
        }
        bug!(cfg, "context cell: expected key to be {CELL_VALUE}, but got {key}");
        None
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Val> {
        if &*key == CELL_VALUE {
            return Some(&mut self.value);
        }
        bug!(cfg, "context cell: expected key to be {CELL_VALUE}, but got {key}");
        None
    }

    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Val) -> Option<()> {
        if &*key == CELL_VALUE {
            self.value = value;
            return Some(());
        }
        bug!(cfg, "context cell: expected key to be {CELL_VALUE}, but got {key}");
        None
    }
}

pub(crate) const QUOTE_VALUE: &str = concatcp!(PREFIX_CELL, QUOTE, ".value");

impl DynCtx<Key, Val> for QuoteVal {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Val> {
        if &*key == QUOTE_VALUE {
            return Some(&self.value);
        }
        bug!(cfg, "context quote: expected key to be {QUOTE_VALUE}, but got {key}");
        None
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Val> {
        if &*key == QUOTE_VALUE {
            return Some(&mut self.value);
        }
        bug!(cfg, "context quote: expected key to be {QUOTE_VALUE}, but got {key}");
        None
    }

    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Val) -> Option<()> {
        if &*key == QUOTE_VALUE {
            self.value = value;
            return Some(());
        }
        bug!(cfg, "context quote: expected key to be {QUOTE_VALUE}, but got {key}");
        None
    }
}

pub(crate) const PAIR_LEFT: &str = concatcp!(PREFIX_CELL, PAIR, ".left");
pub(crate) const PAIR_RIGHT: &str = concatcp!(PREFIX_CELL, PAIR, ".right");

impl DynCtx<Key, Val> for PairVal {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Val> {
        match &*key {
            PAIR_LEFT => return Some(&self.left),
            PAIR_RIGHT => return Some(&self.right),
            _ => {},
        }
        bug!(cfg, "context pair: expected key to be {PAIR_LEFT} or {PAIR_RIGHT}, but got {key}");
        None
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Val> {
        match &*key {
            PAIR_LEFT => return Some(&mut self.left),
            PAIR_RIGHT => return Some(&mut self.right),
            _ => {},
        }
        bug!(cfg, "context pair: expected key to be {PAIR_LEFT} or {PAIR_RIGHT}, but got {key}");
        None
    }

    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Val) -> Option<()> {
        match &*key {
            PAIR_LEFT => {
                self.left = value;
                return Some(());
            },
            PAIR_RIGHT => {
                self.right = value;
                return Some(());
            },
            _ => {},
        }
        bug!(cfg, "context pair: expected key to be {PAIR_LEFT} or {PAIR_RIGHT}, but got {key}");
        None
    }
}

pub(crate) const CALL_FUNCTION: &str = concatcp!(PREFIX_CELL, CALL, ".function");
pub(crate) const CALL_INPUT: &str = concatcp!(PREFIX_CELL, CALL, ".input");

impl DynCtx<Key, Val> for CallVal {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Val> {
        match &*key {
            CALL_FUNCTION => return Some(&self.func),
            CALL_INPUT => return Some(&self.input),
            _ => {},
        }
        bug!(cfg, "context call: expected key to be {CALL_FUNCTION} or {CALL_INPUT}, but got {key}");
        None
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Val> {
        match &*key {
            CALL_FUNCTION => return Some(&mut self.func),
            CALL_INPUT => return Some(&mut self.input),
            _ => {},
        }
        bug!(cfg, "context call: expected key to be {CALL_FUNCTION} or {CALL_INPUT}, but got {key}");
        None
    }

    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Val) -> Option<()> {
        match &*key {
            CALL_FUNCTION => {
                self.func = value;
                return Some(());
            },
            CALL_INPUT => {
                self.input = value;
                return Some(());
            },
            _ => {},
        }
        bug!(cfg, "context call: expected key to be {CALL_FUNCTION} or {CALL_INPUT}, but got {key}");
        None
    }
}

pub(crate) const LIST_FIRST: &str = concatcp!(PREFIX_CELL, LIST, ".first");
pub(crate) const LIST_LAST: &str = concatcp!(PREFIX_CELL, LIST, ".last");

impl DynCtx<Key, Val> for ListVal {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Val> {
        match &*key {
            LIST_FIRST => {
                if let Some(first) = self.first() {
                    return Some(first);
                }
                bug!(cfg, "context list: get first item on an empty list");
            },
            LIST_LAST => {
                if let Some(last) = self.last() {
                    return Some(last);
                }
                bug!(cfg, "context list: get last item on an empty list");
            },
            s => {
                bug!(cfg, "context list: expected key to be {LIST_FIRST} or {LIST_LAST}, but got {s}");
            },
        }
        None
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Val> {
        match &*key {
            LIST_FIRST => {
                if let Some(first) = self.first_mut() {
                    return Some(first);
                }
                bug!(cfg, "context list: get first item on an empty list");
            },
            LIST_LAST => {
                if let Some(last) = self.last_mut() {
                    return Some(last);
                }
                bug!(cfg, "context list: get last item on an empty list");
            },
            s => {
                bug!(cfg, "context list: expected key to be {LIST_FIRST} or {LIST_LAST}, but got {s}");
            },
        }
        None
    }

    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Val) -> Option<()> {
        match &*key {
            LIST_FIRST => {
                if let Some(first) = self.first_mut() {
                    *first = value;
                    return Some(());
                }
                bug!(cfg, "context list: get first item on an empty list");
            },
            LIST_LAST => {
                if let Some(last) = self.last_mut() {
                    *last = value;
                    return Some(());
                }
                bug!(cfg, "context list: get last item on an empty list");
            },
            s => {
                bug!(cfg, "context list: expected key to be {LIST_FIRST} or {LIST_LAST}, but got {s}");
            },
        }
        None
    }
}

impl DynCtx<Key, Val> for MapVal {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Val> {
        if let Some(val) = self.get(&key) {
            return Some(val);
        }
        bug!(cfg, "context map: value not found for key {key}");
        None
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Val> {
        if let Some(val) = self.get_mut(&key) {
            return Some(val);
        }
        bug!(cfg, "context map: value not found for key {key}");
        None
    }

    fn set(&mut self, _cfg: &mut Cfg, key: Key, value: Val) -> Option<()> {
        self.insert(key, value);
        Some(())
    }
}

impl DynCtx<Key, Val> for Val {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Val> {
        match self {
            Val::Cell(cell) => cell.ref_(cfg, key),
            Val::Pair(pair) => pair.ref_(cfg, key),
            Val::List(list) => list.ref_(cfg, key),
            Val::Map(map) => map.ref_(cfg, key),
            Val::Quote(quote) => quote.ref_(cfg, key),
            Val::Call(call) => call.ref_(cfg, key),
            Val::Dyn(val) => val.ref_(cfg, Val::Key(key)),
            v => {
                bug!(cfg, "context: value not found for key {key} in {v}");
                None
            },
        }
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Val> {
        match self {
            Val::Cell(cell) => cell.ref_mut(cfg, key),
            Val::Pair(pair) => pair.ref_mut(cfg, key),
            Val::List(list) => list.ref_mut(cfg, key),
            Val::Map(map) => map.ref_mut(cfg, key),
            Val::Quote(quote) => quote.ref_mut(cfg, key),
            Val::Call(call) => call.ref_mut(cfg, key),
            Val::Dyn(val) => val.ref_mut(cfg, Val::Key(key)),
            v => {
                bug!(cfg, "context: value not found for key {key} in {v}");
                None
            },
        }
    }

    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Val) -> Option<()> {
        match self {
            Val::Cell(cell) => cell.set(cfg, key, value),
            Val::Pair(pair) => pair.set(cfg, key, value),
            Val::List(list) => list.set(cfg, key, value),
            Val::Map(map) => map.set(cfg, key, value),
            Val::Quote(quote) => quote.set(cfg, key, value),
            Val::Call(call) => call.set(cfg, key, value),
            Val::Dyn(val) => val.set(cfg, Val::Key(key), value),
            v => {
                bug!(cfg, "context: value not found for key {key} in {v}");
                None
            },
        }
    }
}

impl DynCtx<IntVal, Val> for ListVal {
    fn ref_(&self, cfg: &mut Cfg, key: IntVal) -> Option<&Val> {
        let len = self.len();
        let Some(index) = key.to_usize() else {
            bug!(cfg, "context list: key {key} should >= 0 and < list.length {len}");
            return None;
        };
        let Some(val) = self.get(index) else {
            bug!(cfg, "context list: key {index} should < list.length {len}");
            return None;
        };
        Some(val)
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: IntVal) -> Option<&mut Val> {
        let len = self.len();
        let Some(index) = key.to_usize() else {
            bug!(cfg, "context list: key {key} should >= 0 and < list.length {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            bug!(cfg, "context list: key {index} should < list.length {len}");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, cfg: &mut Cfg, key: IntVal, value: Val) -> Option<()> {
        let len = self.len();
        let Some(index) = key.to_usize() else {
            bug!(cfg, "context list: key {key} should >= 0 and < list.length {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            bug!(cfg, "context list: key {index} should < list.length {len}");
            return None;
        };
        *val = value;
        Some(())
    }
}

impl DynCtx<Val, Val> for Val {
    fn ref_(&self, cfg: &mut Cfg, key: Val) -> Option<&Val> {
        if let Val::Key(name) = &key {
            return self.ref_(cfg, name.clone());
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = key else {
                    bug!(cfg, "context list: key {key} should be an integer");
                    return None;
                };
                list.ref_(cfg, index)
            },
            Val::Dyn(val) => val.ref_(cfg, key),
            _ => {
                bug!(cfg, "context: value not found for key {key} in {self}");
                None
            },
        }
    }

    fn ref_mut(&mut self, cfg: &mut Cfg, key: Val) -> Option<&mut Val> {
        if let Val::Key(name) = &key {
            return self.ref_mut(cfg, name.clone());
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = key else {
                    bug!(cfg, "context list: key {key} should be an integer");
                    return None;
                };
                list.ref_mut(cfg, index)
            },
            Val::Dyn(val) => val.ref_mut(cfg, key),
            _ => {
                bug!(cfg, "context: value not found for key {key} in {self}");
                None
            },
        }
    }

    fn set(&mut self, cfg: &mut Cfg, key: Val, value: Val) -> Option<()> {
        if let Val::Key(name) = &key {
            return self.set(cfg, name.clone(), value);
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = key else {
                    bug!(cfg, "context list: key {key} should be an integer");
                    return None;
                };
                list.set(cfg, index, value)
            },
            Val::Dyn(val) => val.set(cfg, key, value),
            _ => {
                bug!(cfg, "context: value not found for key {key} in {self}");
                None
            },
        }
    }
}
