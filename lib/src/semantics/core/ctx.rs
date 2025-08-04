use log::error;
use num_traits::ToPrimitive;

use crate::semantics::ctx::Contract;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Symbol;

pub(crate) fn symbol_ref(ctx: &mut Val, name: Symbol) -> Option<DynRef<'_, Val>> {
    match ctx {
        Val::Pair(pair) => match &*name {
            "first" => Some(DynRef::new_mut(&mut pair.first)),
            "second" => Some(DynRef::new_mut(&mut pair.second)),
            _ => {
                error!("symbol {name:?} should be first or second");
                None
            }
        },
        Val::Task(task) => match &*name {
            "function" => Some(DynRef::new_mut(&mut task.func)),
            "context" => Some(DynRef::new_mut(&mut task.ctx)),
            "input" => Some(DynRef::new_mut(&mut task.input)),
            _ => {
                error!("symbol {name:?} should be function, context or input");
                None
            }
        },
        Val::List(list) => {
            let len = list.len();
            let Ok(index) = name.parse::<usize>() else {
                error!("symbol {name:?} should be a int and >= 0 and < list.len {len}");
                return None;
            };
            let Some(val) = list.get_mut(index) else {
                error!("index {index} should < list.len {len}");
                return None;
            };
            Some(DynRef::new_mut(val))
        }
        Val::Map(map) => {
            let Some(val) = map.get_mut(&Val::Symbol(name.clone())) else {
                error!("name {name:?} should exist in the map");
                return None;
            };
            Some(DynRef::new_mut(val))
        }
        Val::Ctx(ctx) => {
            let Ok(val) = ctx.get_ref_dyn(name.clone()) else {
                error!("name {name:?} should exist");
                return None;
            };
            Some(val)
        }
        Val::Dyn(val) => val.ref_(&Val::Symbol(name)),
        v => {
            error!("symbol {name:?} should exist in {v:?}");
            None
        }
    }
}

pub(crate) fn const_ctx_ref(ctx: ConstRef<Val>, input: Val) -> Option<ConstRef<Val>> {
    mut_ctx_ref(ctx.unwrap(), input).map(DynRef::into_const)
}

pub(crate) fn mut_ctx_ref(ctx: &mut Val, input: Val) -> Option<DynRef<'_, Val>> {
    match &input {
        Val::Unit(_) => return Some(DynRef::new_mut(ctx)),
        Val::Symbol(name) => return symbol_ref(ctx, name.clone()),
        _ => {}
    }
    match ctx {
        Val::List(list) => {
            let Val::Int(index) = input else {
                error!("ref {input:?} should be a int");
                return None;
            };
            let len = list.len();
            let Some(index) = index.to_usize() else {
                error!("index {index:?} should >= 0 and < list.len {len}");
                return None;
            };
            let Some(val) = list.get_mut(index) else {
                error!("index {index} should < list.len {len}");
                return None;
            };
            Some(DynRef::new_mut(val))
        }
        Val::Map(map) => {
            let Some(val) = map.get_mut(&input) else {
                error!("ref {input:?} should exist in the map");
                return None;
            };
            Some(DynRef::new_mut(val))
        }
        Val::Dyn(val) => val.ref_(&input),
        _ => {
            error!("ctx {ctx:?} should be a pair, a task, a list, a map or a ctx");
            None
        }
    }
}

pub(crate) fn with_lock<F>(ctx: &mut Val, func_name: Symbol, f: F) -> Val
where F: FnOnce(&mut Val, FuncVal, Contract) -> (FuncVal, Val) {
    // todo design support lock in other type ctx
    let Val::Ctx(ctx_val) = ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    let Ok(ctx_value) = ctx_val.lock(func_name.clone()) else {
        error!("func ref {func_name:?} should be lockable");
        return Val::default();
    };
    let Val::Func(func) = ctx_value.val else {
        error!("func ref {:?} should be a func", ctx_value.val);
        ctx_val.unlock(func_name, ctx_value.val);
        return Val::default();
    };
    let (func, output) = f(ctx, func, ctx_value.contract);
    let Val::Ctx(ctx_val) = ctx else {
        unreachable!("lock_unlock ctx invariant is broken!!!");
    };
    ctx_val.unlock(func_name, Val::Func(func));
    output
}
