use std::mem::take;

use log::error;
use num_traits::ToPrimitive;

use crate::semantics::ctx::DynCtx;
use crate::semantics::val::CtxVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::IntVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
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
}

impl DynCtx<Symbol, Val> for TaskVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        match &*input {
            "function" => Some(DynRef::new_mut(&mut self.func)),
            "context" => Some(DynRef::new_mut(&mut self.ctx)),
            "input" => Some(DynRef::new_mut(&mut self.input)),
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
}

impl DynCtx<Symbol, Val> for MapVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        let Some(val) = self.get_mut(&Val::Symbol(input.clone())) else {
            error!("name {input:?} should exist in the map");
            return None;
        };
        Some(DynRef::new_mut(val))
    }
}

impl DynCtx<Symbol, Val> for CtxVal {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        let Ok(val) = self.get_ref_dyn(input.clone()) else {
            error!("name {input:?} should exist");
            return None;
        };
        Some(val)
    }
}

impl DynCtx<Symbol, Val> for Val {
    fn ref_(&mut self, input: Symbol) -> Option<DynRef<'_, Val>> {
        match self {
            Val::Pair(pair) => pair.ref_(input),
            Val::Task(task) => task.ref_(input),
            Val::List(list) => list.ref_(input),
            Val::Map(map) => map.ref_(input),
            Val::Ctx(ctx) => ctx.ref_(input),
            Val::Dyn(val) => val.ref_(Val::Symbol(input)),
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
}

impl DynCtx<Val, Val> for MapVal {
    fn ref_(&mut self, input: Val) -> Option<DynRef<'_, Val>> {
        let Some(val) = self.get_mut(&input) else {
            error!("ref {input:?} should exist in the map");
            return None;
        };
        Some(DynRef::new_mut(val))
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
                error!("ctx {self:?} should be a pair, a task, a list, a map or a ctx");
                None
            }
        }
    }
}

pub(crate) fn const_ctx_ref(ctx: ConstRef<Val>, input: Val) -> Option<ConstRef<Val>> {
    ctx.unwrap().ref_(input).map(DynRef::into_const)
}

pub(crate) fn mut_ctx_ref(ctx: &mut Val, input: Val) -> Option<DynRef<'_, Val>> {
    ctx.ref_(input)
}

pub(crate) fn func_ref<F>(ctx: &mut Val, func_name: Symbol, f: F) -> Val
where F: FnOnce(&mut Val, DynRef<FuncVal>) -> Val {
    match ctx {
        Val::Ctx(ctx_val) => {
            let Ok(ctx_value) = ctx_val.lock(func_name.clone()) else {
                error!("func ref {func_name:?} should be lockable");
                return Val::default();
            };
            let Val::Func(mut func) = ctx_value.val else {
                error!("func ref {:?} should be a func", ctx_value.val);
                ctx_val.unlock(func_name, ctx_value.val);
                return Val::default();
            };
            let output = f(ctx, DynRef::new(&mut func, !ctx_value.contract.is_mutable()));
            let Val::Ctx(ctx_val) = ctx else {
                unreachable!("func_ref ctx invariant is broken!!!");
            };
            ctx_val.unlock(func_name, Val::Func(func));
            output
        }
        ctx => {
            let Some(v) = ctx.ref_(func_name.clone()) else {
                return Val::default();
            };
            let v = v.unwrap();
            let Val::Func(func) = v else {
                error!("func ref {v:?} should be a func");
                return Val::default();
            };
            let mut func_ref = take(func);
            let output = f(ctx, DynRef::new_mut(&mut func_ref));
            let Some(v) = ctx.ref_(func_name) else {
                unreachable!("func_ref ctx invariant is broken!!!");
            };
            *v.unwrap() = Val::Func(func_ref);
            output
        }
    }
}
