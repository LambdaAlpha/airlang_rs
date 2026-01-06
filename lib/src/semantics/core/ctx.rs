use std::mem::swap;

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
            error!("key {key:?} should be value");
            None
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        if &*key == VALUE {
            Some(&mut self.value)
        } else {
            error!("key {key:?} should be value");
            None
        }
    }

    fn set(&mut self, key: Key, mut value: Val) -> Option<Val> {
        if &*key == VALUE {
            swap(&mut self.value, &mut value);
            Some(value)
        } else {
            error!("key {key:?} should be value");
            None
        }
    }
}

const FIRST: &str = concatcp!(PREFIX_ID, "first");
const SECOND: &str = concatcp!(PREFIX_ID, "second");

impl DynCtx<Key, Val> for PairVal {
    fn ref_(&self, key: Key) -> Option<&Val> {
        match &*key {
            FIRST => Some(&self.first),
            SECOND => Some(&self.second),
            _ => {
                error!("key {key:?} should be first or second");
                None
            }
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        match &*key {
            FIRST => Some(&mut self.first),
            SECOND => Some(&mut self.second),
            _ => {
                error!("key {key:?} should be first or second");
                None
            }
        }
    }

    fn set(&mut self, key: Key, mut value: Val) -> Option<Val> {
        match &*key {
            FIRST => {
                swap(&mut self.first, &mut value);
                Some(value)
            }
            SECOND => {
                swap(&mut self.second, &mut value);
                Some(value)
            }
            _ => {
                error!("key {key:?} should be first or second");
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
                error!("key {key:?} should be function or input");
                None
            }
        }
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        match &*key {
            FUNCTION => Some(&mut self.func),
            INPUT => Some(&mut self.input),
            _ => {
                error!("key {key:?} should be function or input");
                None
            }
        }
    }

    fn set(&mut self, key: Key, mut value: Val) -> Option<Val> {
        match &*key {
            FUNCTION => {
                swap(&mut self.func, &mut value);
                Some(value)
            }
            INPUT => {
                swap(&mut self.input, &mut value);
                Some(value)
            }
            _ => {
                error!("key {key:?} should be function or input");
                None
            }
        }
    }
}

impl DynCtx<Key, Val> for ListVal {
    fn ref_(&self, key: Key) -> Option<&Val> {
        let len = self.len();
        let Ok(index) = key.parse::<usize>() else {
            error!("key {key:?} should be a int and >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get(index) else {
            error!("key {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn ref_mut(&mut self, key: Key) -> Option<&mut Val> {
        let len = self.len();
        let Ok(index) = key.parse::<usize>() else {
            error!("key {key:?} should be a int and >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("key {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, key: Key, mut value: Val) -> Option<Val> {
        let len = self.len();
        let Ok(index) = key.parse::<usize>() else {
            error!("key {key:?} should be a int and >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("key {index} should < list.len {len}");
            return None;
        };
        swap(val, &mut value);
        Some(value)
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

    fn set(&mut self, key: Key, value: Val) -> Option<Val> {
        self.insert(key, value)
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

    fn set(&mut self, key: Key, value: Val) -> Option<Val> {
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

    fn set(&mut self, key: IntVal, mut value: Val) -> Option<Val> {
        let len = self.len();
        let Some(index) = key.to_usize() else {
            error!("key {key:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("key {index} should < list.len {len}");
            return None;
        };
        swap(val, &mut value);
        Some(value)
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

    fn set(&mut self, key: Val, value: Val) -> Option<Val> {
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
