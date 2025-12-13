use std::mem::swap;

use log::error;
use num_traits::ToPrimitive;

use crate::semantics::ctx::DynCtx;
use crate::semantics::val::CallVal;
use crate::semantics::val::IntVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::Key;

const FIRST: &str = "first";
const SECOND: &str = "second";

impl DynCtx<Key, Val> for PairVal {
    fn ref_(&self, input: Key) -> Option<&Val> {
        match &*input {
            FIRST => Some(&self.first),
            SECOND => Some(&self.second),
            _ => {
                error!("key {input:?} should be first or second");
                None
            }
        }
    }

    fn ref_mut(&mut self, input: Key) -> Option<&mut Val> {
        match &*input {
            FIRST => Some(&mut self.first),
            SECOND => Some(&mut self.second),
            _ => {
                error!("key {input:?} should be first or second");
                None
            }
        }
    }

    fn set(&mut self, input: Key, mut value: Val) -> Option<Val> {
        match &*input {
            FIRST => {
                swap(&mut self.first, &mut value);
                Some(value)
            }
            SECOND => {
                swap(&mut self.second, &mut value);
                Some(value)
            }
            _ => {
                error!("key {input:?} should be first or second");
                None
            }
        }
    }
}

const FUNCTION: &str = "function";
const INPUT: &str = "input";

impl DynCtx<Key, Val> for CallVal {
    fn ref_(&self, input: Key) -> Option<&Val> {
        match &*input {
            FUNCTION => Some(&self.func),
            INPUT => Some(&self.input),
            _ => {
                error!("key {input:?} should be function, context or input");
                None
            }
        }
    }

    fn ref_mut(&mut self, input: Key) -> Option<&mut Val> {
        match &*input {
            FUNCTION => Some(&mut self.func),
            INPUT => Some(&mut self.input),
            _ => {
                error!("key {input:?} should be function, context or input");
                None
            }
        }
    }

    fn set(&mut self, input: Key, mut value: Val) -> Option<Val> {
        match &*input {
            FUNCTION => {
                swap(&mut self.func, &mut value);
                Some(value)
            }
            INPUT => {
                swap(&mut self.input, &mut value);
                Some(value)
            }
            _ => {
                error!("key {input:?} should be function, context or input");
                None
            }
        }
    }
}

impl DynCtx<Key, Val> for ListVal {
    fn ref_(&self, input: Key) -> Option<&Val> {
        let len = self.len();
        let Ok(index) = input.parse::<usize>() else {
            error!("key {input:?} should be a int and >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn ref_mut(&mut self, input: Key) -> Option<&mut Val> {
        let len = self.len();
        let Ok(index) = input.parse::<usize>() else {
            error!("key {input:?} should be a int and >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, input: Key, mut value: Val) -> Option<Val> {
        let len = self.len();
        let Ok(index) = input.parse::<usize>() else {
            error!("key {input:?} should be a int and >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        swap(val, &mut value);
        Some(value)
    }
}

impl DynCtx<Key, Val> for MapVal {
    fn ref_(&self, input: Key) -> Option<&Val> {
        let Some(val) = self.get(&input) else {
            error!("name {input:?} should exist in the map");
            return None;
        };
        Some(val)
    }

    fn ref_mut(&mut self, input: Key) -> Option<&mut Val> {
        let Some(val) = self.get_mut(&input) else {
            error!("name {input:?} should exist in the map");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, input: Key, value: Val) -> Option<Val> {
        self.insert(input, value)
    }
}

impl DynCtx<Key, Val> for Val {
    fn ref_(&self, input: Key) -> Option<&Val> {
        match self {
            Val::Pair(pair) => pair.ref_(input),
            Val::Call(call) => call.ref_(input),
            Val::List(list) => list.ref_(input),
            Val::Map(map) => map.ref_(input),
            Val::Dyn(val) => val.ref_(Val::Key(input)),
            v => {
                error!("key {input:?} should exist in {v:?}");
                None
            }
        }
    }

    fn ref_mut(&mut self, input: Key) -> Option<&mut Val> {
        match self {
            Val::Pair(pair) => pair.ref_mut(input),
            Val::Call(call) => call.ref_mut(input),
            Val::List(list) => list.ref_mut(input),
            Val::Map(map) => map.ref_mut(input),
            Val::Dyn(val) => val.ref_mut(Val::Key(input)),
            v => {
                error!("key {input:?} should exist in {v:?}");
                None
            }
        }
    }

    fn set(&mut self, input: Key, value: Val) -> Option<Val> {
        match self {
            Val::Pair(pair) => pair.set(input, value),
            Val::Call(call) => call.set(input, value),
            Val::List(list) => list.set(input, value),
            Val::Map(map) => map.set(input, value),
            Val::Dyn(val) => val.set(Val::Key(input), value),
            v => {
                error!("key {input:?} should exist in {v:?}");
                None
            }
        }
    }
}

impl DynCtx<IntVal, Val> for ListVal {
    fn ref_(&self, input: IntVal) -> Option<&Val> {
        let len = self.len();
        let Some(index) = input.to_usize() else {
            error!("index {input:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn ref_mut(&mut self, input: IntVal) -> Option<&mut Val> {
        let len = self.len();
        let Some(index) = input.to_usize() else {
            error!("index {input:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, input: IntVal, mut value: Val) -> Option<Val> {
        let len = self.len();
        let Some(index) = input.to_usize() else {
            error!("index {input:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        swap(val, &mut value);
        Some(value)
    }
}

impl DynCtx<Val, Val> for Val {
    fn ref_(&self, input: Val) -> Option<&Val> {
        if let Val::Key(name) = &input {
            return self.ref_(name.clone());
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = input else {
                    error!("ref {input:?} should be a int");
                    return None;
                };
                list.ref_(index)
            }
            Val::Dyn(val) => val.ref_(input),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }

    fn ref_mut(&mut self, input: Val) -> Option<&mut Val> {
        if let Val::Key(name) = &input {
            return self.ref_mut(name.clone());
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = input else {
                    error!("ref {input:?} should be a int");
                    return None;
                };
                list.ref_mut(index)
            }
            Val::Dyn(val) => val.ref_mut(input),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }

    fn set(&mut self, input: Val, value: Val) -> Option<Val> {
        if let Val::Key(name) = &input {
            return self.set(name.clone(), value);
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = input else {
                    error!("ref {input:?} should be a int");
                    return None;
                };
                list.set(index, value)
            }
            Val::Dyn(val) => val.set(input, value),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }
}
