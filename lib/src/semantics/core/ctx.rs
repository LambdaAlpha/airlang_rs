use std::mem::swap;

use log::error;
use num_traits::ToPrimitive;

use crate::semantics::ctx::DynCtx;
use crate::semantics::memo::Contract;
use crate::semantics::val::CallVal;
use crate::semantics::val::IntVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::MemoVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::DynRef;
use crate::type_::Symbol;

impl DynCtx<Symbol, Val> for PairVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        match &*input {
            "first" => Some(DynRef::new_mut(&mut self.first)),
            "second" => Some(DynRef::new_mut(&mut self.second)),
            _ => {
                error!("symbol {input:?} should be first or second");
                None
            }
        }
    }

    fn set(&mut self, input: Symbol, mut value: Val) -> Option<Val> {
        match &*input {
            "first" => {
                swap(&mut self.first, &mut value);
                Some(value)
            }
            "second" => {
                swap(&mut self.second, &mut value);
                Some(value)
            }
            _ => {
                error!("symbol {input:?} should be first or second");
                None
            }
        }
    }
}

impl DynCtx<Symbol, Val> for CallVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        match &*input {
            "function" => Some(DynRef::new_mut(&mut self.func)),
            "input" => Some(DynRef::new_mut(&mut self.input)),
            _ => {
                error!("symbol {input:?} should be function, context or input");
                None
            }
        }
    }

    fn set(&mut self, input: Symbol, mut value: Val) -> Option<Val> {
        match &*input {
            "function" => {
                swap(&mut self.func, &mut value);
                Some(value)
            }
            "input" => {
                swap(&mut self.input, &mut value);
                Some(value)
            }
            _ => {
                error!("symbol {input:?} should be function, context or input");
                None
            }
        }
    }
}

impl DynCtx<Symbol, Val> for ListVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        let len = self.len();
        let Ok(index) = input.parse::<usize>() else {
            error!("symbol {input:?} should be a int and >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        Some(DynRef::new_mut(val))
    }

    fn set(&mut self, input: Symbol, mut value: Val) -> Option<Val> {
        let len = self.len();
        let Ok(index) = input.parse::<usize>() else {
            error!("symbol {input:?} should be a int and >= 0 and < list.len {len}");
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

impl DynCtx<Symbol, Val> for MapVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        let Some(val) = self.get_mut(&Val::Symbol(input.clone())) else {
            error!("name {input:?} should exist in the map");
            return None;
        };
        Some(DynRef::new_mut(val))
    }

    fn set(&mut self, input: Symbol, value: Val) -> Option<Val> {
        self.insert(Val::Symbol(input), value)
    }
}

impl DynCtx<Symbol, Val> for MemoVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        let Ok(val) = self.get_ref_dyn(input.clone()) else {
            error!("name {input:?} should exist");
            return None;
        };
        Some(val)
    }

    fn set(&mut self, input: Symbol, value: Val) -> Option<Val> {
        let Ok(last) = self.put(input.clone(), value, Contract::None) else {
            error!("variable {input:?} is not assignable");
            return None;
        };
        last
    }
}

impl DynCtx<Symbol, Val> for Val {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        match self {
            Val::Pair(pair) => pair.ref_(input),
            Val::Call(call) => call.ref_(input),
            Val::List(list) => list.ref_(input),
            Val::Map(map) => map.ref_(input),
            Val::Memo(ctx) => ctx.ref_(input),
            Val::Dyn(val) => val.ref_(Val::Symbol(input)),
            v => {
                error!("symbol {input:?} should exist in {v:?}");
                None
            }
        }
    }

    fn set(&mut self, input: Symbol, value: Val) -> Option<Val> {
        match self {
            Val::Pair(pair) => pair.set(input, value),
            Val::Call(call) => call.set(input, value),
            Val::List(list) => list.set(input, value),
            Val::Map(map) => map.set(input, value),
            Val::Memo(ctx) => ctx.set(input, value),
            Val::Dyn(val) => val.set(Val::Symbol(input), value),
            v => {
                error!("symbol {input:?} should exist in {v:?}");
                None
            }
        }
    }
}

impl DynCtx<IntVal, Val> for ListVal {
    fn ref_(&mut self, input: IntVal) -> Option<DynRef<'_, Val>> {
        let len = self.len();
        let Some(index) = input.to_usize() else {
            error!("index {input:?} should >= 0 and < list.len {len}");
            return None;
        };
        let Some(val) = self.get_mut(index) else {
            error!("index {index} should < list.len {len}");
            return None;
        };
        Some(DynRef::new_mut(val))
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

impl DynCtx<Val, Val> for MapVal {
    fn ref_(&mut self, input: Val) -> Option<DynRef<'_, Val>> {
        let Some(val) = self.get_mut(&input) else {
            error!("ref {input:?} should exist in the map");
            return None;
        };
        Some(DynRef::new_mut(val))
    }

    fn set(&mut self, input: Val, value: Val) -> Option<Val> {
        self.insert(input, value)
    }
}

impl DynCtx<Val, Val> for Val {
    fn ref_(&mut self, input: Val) -> Option<DynRef<'_, Val>> {
        match &input {
            Val::Unit(_) => return Some(DynRef::new_mut(self)),
            Val::Symbol(name) => return self.ref_(name.clone()),
            _ => {}
        }
        match self {
            Val::List(list) => {
                let Val::Int(index) = input else {
                    error!("ref {input:?} should be a int");
                    return None;
                };
                list.ref_(index)
            }
            Val::Map(map) => map.ref_(input),
            Val::Dyn(val) => val.ref_(input),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }

    fn set(&mut self, input: Val, value: Val) -> Option<Val> {
        if let Val::Symbol(name) = &input {
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
            Val::Map(map) => map.set(input, value),
            Val::Dyn(val) => val.set(input, value),
            _ => {
                error!("ctx {self:?} should be a dyn ctx");
                None
            }
        }
    }
}
