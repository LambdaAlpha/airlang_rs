use std::matches;

use crate::{
    ctx::{
        map::CtxMapRef,
        mut1::MutFnCtx,
        ref1::{
            CtxMeta,
            CtxRef,
        },
        CtxValue,
        DynRef,
    },
    Ctx,
    CtxError,
    FreeCtx,
    FuncVal,
    Symbol,
    Val,
};

/*
Why `&mut Ctx`? What we actually need is an owned `Ctx`, because we need to store the ctx when
evaluating a ctx-aware function. But a `&mut Ctx` is more compact and convenient, and we can
change `&mut Ctx` back to `Ctx` at anytime we need by swapping its memory with a default ctx.
The `const` is just a flag and a runtime invariant.
*/
pub struct ConstCtx<'a>(&'a mut Ctx);

pub enum ConstFnCtx<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
}

impl<'l> CtxMapRef<'l> for ConstCtx<'l> {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        self.0.get_ref(name)
    }

    fn get_ref_mut(self, _name: Symbol) -> Result<&'l mut Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        let mut dyn_ref = self.0.get_ref_dyn(name)?;
        dyn_ref.is_const = true;
        Ok(dyn_ref)
    }

    fn remove(self, _name: Symbol) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_assignable(self, name: Symbol) -> bool {
        self.0.is_assignable(name)
    }

    fn put_value(self, _name: Symbol, _value: CtxValue) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        self.0.is_final(name)
    }

    fn set_const(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        self.0.is_const(name)
    }
}

impl<'l> CtxRef<'l> for ConstCtx<'l> {
    fn get_solver(self) -> Result<&'l FuncVal, CtxError> {
        self.0.get_solver()
    }

    fn get_solver_mut(self) -> Result<&'l mut FuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, FuncVal>, CtxError> {
        let mut dyn_ref = self.0.get_solver_dyn()?;
        dyn_ref.is_const = true;
        Ok(dyn_ref)
    }

    fn set_solver(self, _solver: Option<FuncVal>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl<'l> CtxMeta<'l> for ConstCtx<'l> {
    type Reborrow<'s> = ConstCtx<'s> where Self: 's;

    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        ConstCtx(self.0)
    }

    fn borrow(&self) -> Option<&Ctx> {
        Some(self.0)
    }

    fn is_ctx_free(self) -> bool {
        false
    }

    fn is_ctx_const(self) -> bool {
        true
    }

    fn for_const_fn(self) -> ConstFnCtx<'l> {
        ConstFnCtx::Const(self)
    }

    fn for_mut_fn(self) -> MutFnCtx<'l> {
        MutFnCtx::Const(self)
    }
}

impl<'l> CtxMapRef<'l> for ConstFnCtx<'l> {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_ref(name),
            ConstFnCtx::Const(ctx) => <_ as CtxMapRef>::get_ref(ctx, name),
        }
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_ref_mut(name),
            ConstFnCtx::Const(ctx) => ctx.get_ref_mut(name),
        }
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_ref_dyn(name),
            ConstFnCtx::Const(ctx) => ctx.get_ref_dyn(name),
        }
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.remove(name),
            ConstFnCtx::Const(ctx) => ctx.remove(name),
        }
    }

    fn is_assignable(self, name: Symbol) -> bool {
        match self {
            ConstFnCtx::Free(ctx) => ctx.is_assignable(name),
            ConstFnCtx::Const(ctx) => ctx.is_assignable(name),
        }
    }

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.put_value(name, value),
            ConstFnCtx::Const(ctx) => ctx.put_value(name, value),
        }
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.set_final(name),
            ConstFnCtx::Const(ctx) => ctx.set_final(name),
        }
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.is_final(name),
            ConstFnCtx::Const(ctx) => ctx.is_final(name),
        }
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.set_const(name),
            ConstFnCtx::Const(ctx) => ctx.set_const(name),
        }
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.is_const(name),
            ConstFnCtx::Const(ctx) => ctx.is_const(name),
        }
    }
}

impl<'l> CtxRef<'l> for ConstFnCtx<'l> {
    fn get_solver(self) -> Result<&'l FuncVal, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_solver(),
            ConstFnCtx::Const(ctx) => ctx.get_solver(),
        }
    }

    fn get_solver_mut(self) -> Result<&'l mut FuncVal, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_solver_mut(),
            ConstFnCtx::Const(ctx) => ctx.get_solver_mut(),
        }
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, FuncVal>, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_solver_dyn(),
            ConstFnCtx::Const(ctx) => ctx.get_solver_dyn(),
        }
    }

    fn set_solver(self, solver: Option<FuncVal>) -> Result<(), CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.set_solver(solver),
            ConstFnCtx::Const(ctx) => ctx.set_solver(solver),
        }
    }
}

impl<'l> CtxMeta<'l> for ConstFnCtx<'l> {
    type Reborrow<'s> = ConstFnCtx<'s> where 'l: 's;

    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        match self {
            ConstFnCtx::Free(ctx) => ConstFnCtx::Free(ctx.reborrow()),
            ConstFnCtx::Const(ctx) => ConstFnCtx::Const(ctx.reborrow()),
        }
    }

    fn borrow(&self) -> Option<&Ctx> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.borrow(),
            ConstFnCtx::Const(ctx) => ctx.borrow(),
        }
    }

    fn is_ctx_free(self) -> bool {
        matches!(self, ConstFnCtx::Free(_))
    }

    fn is_ctx_const(self) -> bool {
        true
    }

    fn for_const_fn(self) -> ConstFnCtx<'l> {
        self
    }

    fn for_mut_fn(self) -> MutFnCtx<'l> {
        match self {
            ConstFnCtx::Free(_ctx) => MutFnCtx::Free(FreeCtx),
            ConstFnCtx::Const(ctx) => MutFnCtx::Const(ctx),
        }
    }
}

impl<'a> ConstCtx<'a> {
    pub fn new(ctx: &'a mut Ctx) -> Self {
        ConstCtx(ctx)
    }

    pub fn reborrow(&mut self) -> ConstCtx {
        <_ as CtxMeta<'a>>::reborrow(self)
    }

    pub fn borrow(&self) -> Option<&Ctx> {
        <_ as CtxMeta<'a>>::borrow(self)
    }

    pub(crate) fn get_ctx_ref(self) -> &'a Ctx {
        self.0
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        <_ as CtxMapRef>::get_ref(self, name)
    }

    // INVARIANT: The function f can take the ctx out during its execution,
    // but when f returns, ctx must be equal to its original value.
    pub(crate) fn temp_take<'b, T, F>(&'b mut self, f: F) -> T
    where
        F: FnOnce(&'b mut Ctx) -> T,
    {
        f(self.0)
    }
}

impl<'a> ConstFnCtx<'a> {
    pub fn reborrow(&mut self) -> ConstFnCtx {
        <_ as CtxMeta<'a>>::reborrow(self)
    }

    pub fn borrow(&self) -> Option<&Ctx> {
        <_ as CtxMeta<'a>>::borrow(self)
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        <_ as CtxMapRef>::get_ref(self, name)
    }
}
