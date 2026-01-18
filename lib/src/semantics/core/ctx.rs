use const_format::concatcp;
use log::error;
use num_traits::ToPrimitive;

use crate::semantics::core::PREFIX_ID;
use crate::semantics::ctx::DynCtx;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::IntVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::Key;

const VALUE: &str = concatcp!(PREFIX_ID, "value");

impl DynCtx<Key, Val> for CellVal {
    fn ref_(&self, key: Key) -> Option<&Val> {
        if &*key == VALUE {
            Some(&self.value)
        } else {
            error!("key {key:?} should be {VALUE}");
            None
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        if &*key == VALUE {
            Some(&mut self.value)
        } else {
            error!("key {key:?} should be {VALUE}");
            None
        }
    }

    fn set(&mut self, key: Key, value: Val) -> Option<()> {
        if &*key == VALUE {
            self.value = value;
            Some(())
        } else {
            error!("key {key:?} should be {VALUE}");
            None
        }
    }
}

const LEFT: &str = concatcp!(PREFIX_ID, "left");
const RIGHT: &str = concatcp!(PREFIX_ID, "right");

impl DynCtx<Key, Val> for PairVal {
    fn ref_(&self, key: Key) -> Option<&Val> {
        match &*key {
            LEFT => Some(&self.left),
            RIGHT => Some(&self.right),
            _ => {
                error!("key {key:?} should be {LEFT} or {RIGHT}");
                None
            }
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        match &*key {
            LEFT => Some(&mut self.left),
            RIGHT => Some(&mut self.right),
            _ => {
                error!("key {key:?} should be {LEFT} or {RIGHT}");
                None
            }
        }
    }

    fn set(&mut self, key: Key, value: Val) -> Option<()> {
        match &*key {
            LEFT => {
                self.left = value;
                Some(())
            }
            RIGHT => {
                self.right = value;
                Some(())
            }
            _ => {
                error!("key {key:?} should be {LEFT} or {RIGHT}");
                None
            }
        }
    }
}

const FUNCTION: &str = concatcp!(PREFIX_ID, "function");
const INPUT: &str = concatcp!(PREFIX_ID, "input");

impl DynCtx<Key, Val> for CallVal {
    fn ref_(&self, key: Key) -> Option<&Val> {
        match &*key {
            FUNCTION => Some(&self.func),
            INPUT => Some(&self.input),
            _ => {
                error!("key {key:?} should be {FUNCTION} or {INPUT}");
                None
            }
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        match &*key {
            FUNCTION => Some(&mut self.func),
            INPUT => Some(&mut self.input),
            _ => {
                error!("key {key:?} should be {FUNCTION} or {INPUT}");
                None
            }
        }
    }

    fn set(&mut self, key: Key, value: Val) -> Option<()> {
        match &*key {
            FUNCTION => {
                self.func = value;
                Some(())
            }
            INPUT => {
                self.input = value;
                Some(())
            }
            _ => {
                error!("key {key:?} should be {FUNCTION} or {INPUT}");
                None
            }
        }
    }
}

const FIRST: &str = concatcp!(PREFIX_ID, "first");
const LAST: &str = concatcp!(PREFIX_ID, "last");

impl DynCtx<Key, Val> for ListVal {
    fn ref_(&self, key: Key) -> Option<&Val> {
        match &*key {
            FIRST => self.first(),
            LAST => self.last(),
            _ => {
                error!("key {key:?} should be {FIRST} or {LAST}");
                None
            }
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        match &*key {
            FIRST => self.first_mut(),
            LAST => self.last_mut(),
            _ => {
                error!("key {key:?} should be {FIRST} or {LAST}");
                None
            }
        }
    }

    fn set(&mut self, key: Key, value: Val) -> Option<()> {
        match &*key {
            FIRST => {
                *self.first_mut()? = value;
                Some(())
            }
            LAST => {
                *self.last_mut()? = value;
                Some(())
            }
            _ => {
                error!("key {key:?} should be {FIRST} or {LAST}");
                None
            }
        }
    }
}

impl DynCtx<Key, Val> for MapVal {
    fn ref_(&self, key: Key) -> Option<&Val> {
        let Some(val) = self.get(&key) else {
            error!("key {key:?} should exist in the map");
            return None;
        };
        Some(val)
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        let Some(val) = self.get_mut(&key) else {
            error!("key {key:?} should exist in the map");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, key: Key, value: Val) -> Option<()> {
        self.insert(key, value);
        Some(())
    }
}

impl DynCtx<Key, Val> for Val {
    fn ref_(&self, key: Key) -> Option<&Val> {
        match self {
            Val::Cell(cell) => cell.ref_(key),
            Val::Pair(pair) => pair.ref_(key),
            Val::Call(call) => call.ref_(key),
            Val::List(list) => list.ref_(key),
            Val::Map(map) => map.ref_(key),
            Val::Dyn(val) => val.ref_(Val::Key(key)),
            v => {
                error!("key {key:?} should exist in {v:?}");
                None
            }
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        match self {
            Val::Cell(cell) => cell.ref_mut(key),
            Val::Pair(pair) => pair.ref_mut(key),
            Val::Call(call) => call.ref_mut(key),
            Val::List(list) => list.ref_mut(key),
            Val::Map(map) => map.ref_mut(key),
            Val::Dyn(val) => val.ref_mut(Val::Key(key)),
            v => {
                error!("key {key:?} should exist in {v:?}");
                None
            }
        }
    }

    fn set(&mut self, key: Key, value: Val) -> Option<()> {
        match self {
            Val::Cell(cell) => cell.set(key, value),
            Val::Pair(pair) => pair.set(key, value),
            Val::Call(call) => call.set(key, value),
            Val::List(list) => list.set(key, value),
            Val::Map(map) => map.set(key, value),
            Val::Dyn(val) => val.set(Val::Key(key), value),
            v => {
                error!("key {key:?} should exist in {v:?}");
                None
            }
        }
    }
}

impl DynCtx<IntVal, Val> for ListVal {
    fn ref_(&self, key: IntVal) -> Option<&Val> {
        let len = self.len();
        let Some(index) = key.to_usize() else {
            error!("key {key:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get(index) else {
            error!("key {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn ref_mut(&mut self, key: IntVal) -> Option<&mut Val> {
        let len = self.len();
        let Some(index) = key.to_usize() else {
            error!("key {key:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("key {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, key: IntVal, value: Val) -> Option<()> {
        let len = self.len();
        let Some(index) = key.to_usize() else {
            error!("key {key:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("key {index} should < list.len {len}");
            return None;
        };
        *val = value;
        Some(())
    }
}

impl DynCtx<Val, Val> for Val {
    fn ref_(&self, key: Val) -> Option<&Val> {
        if let Val::Key(name) = &key {
            return self.ref_(name.clone());
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = key else {
                    error!("key {key:?} should be a int");
                    return None;
                };
                list.ref_(index)
            }
            Val::Dyn(val) => val.ref_(key),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }

    fn ref_mut(&mut self, key: Val) -> Option<&mut Val> {
        if let Val::Key(name) = &key {
            return self.ref_mut(name.clone());
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = key else {
                    error!("key {key:?} should be a int");
                    return None;
                };
                list.ref_mut(index)
            }
            Val::Dyn(val) => val.ref_mut(key),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }

    fn set(&mut self, key: Val, value: Val) -> Option<()> {
        if let Val::Key(name) = &key {
            return self.set(name.clone(), value);
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = key else {
                    error!("key {key:?} should be a int");
                    return None;
                };
                list.set(index, value)
            }
            Val::Dyn(val) => val.set(key, value),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }
}
