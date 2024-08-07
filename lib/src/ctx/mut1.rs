use std::{
    matches,
    mem::swap,
};

use crate::{
    ctx::{
        map::CtxMapRef,
        ref1::{
            CtxMeta,
            CtxRef,
        },
        CtxValue,
        DynRef,
    },
    ConstCtx,
    ConstFnCtx,
    Ctx,
    CtxError,
    FreeCtx,
    FuncVal,
    Invariant,
    Symbol,
    Val,
};

pub struct MutCtx<'a>(&'a mut Ctx);

pub enum MutFnCtx<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
    Mut(MutCtx<'a>),
}

impl<'l> CtxMapRef<'l> for MutCtx<'l> {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        self.0.get_ref(name)
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        self.0.get_ref_mut(name)
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        self.0.get_ref_dyn(name)
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        self.0.remove(name)
    }

    fn is_assignable(self, name: Symbol) -> bool {
        self.0.is_assignable(name)
    }

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        self.0.put_value(name, value)
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        self.0.set_final(name)
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        self.0.is_final(name)
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        self.0.set_const(name)
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        self.0.is_const(name)
    }
}

impl<'l> CtxRef<'l> for MutCtx<'l> {
    fn get_solver(self) -> Result<&'l FuncVal, CtxError> {
        self.0.get_solver()
    }

    fn get_solver_mut(self) -> Result<&'l mut FuncVal, CtxError> {
        self.0.get_solver_mut()
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, FuncVal>, CtxError> {
        self.0.get_solver_dyn()
    }

    fn set_solver(self, solver: Option<FuncVal>) -> Result<(), CtxError> {
        self.0.set_solver(solver)
    }
}

impl<'l> CtxMeta<'l> for MutCtx<'l> {
    type Reborrow<'s> = MutCtx<'s> where Self: 's;
    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        MutCtx(self.0)
    }

    fn borrow(&self) -> Option<&Ctx> {
        Some(self.0)
    }

    fn is_ctx_free(self) -> bool {
        false
    }
    fn is_ctx_const(self) -> bool {
        false
    }
    fn for_const_fn(self) -> ConstFnCtx<'l> {
        ConstFnCtx::Const(ConstCtx::new(self.0))
    }
    fn for_mut_fn(self) -> MutFnCtx<'l> {
        MutFnCtx::Mut(self)
    }
}

impl<'l> CtxMapRef<'l> for MutFnCtx<'l> {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_ref(name),
            MutFnCtx::Const(ctx) => <_ as CtxMapRef>::get_ref(ctx, name),
            MutFnCtx::Mut(ctx) => <_ as CtxMapRef>::get_ref(ctx, name),
        }
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_ref_mut(name),
            MutFnCtx::Const(ctx) => ctx.get_ref_mut(name),
            MutFnCtx::Mut(ctx) => <_ as CtxMapRef>::get_ref_mut(ctx, name),
        }
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_ref_dyn(name),
            MutFnCtx::Const(ctx) => ctx.get_ref_dyn(name),
            MutFnCtx::Mut(ctx) => ctx.get_ref_dyn(name),
        }
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.remove(name),
            MutFnCtx::Const(ctx) => ctx.remove(name),
            MutFnCtx::Mut(ctx) => ctx.remove(name),
        }
    }

    fn is_assignable(self, name: Symbol) -> bool {
        match self {
            MutFnCtx::Free(ctx) => ctx.is_assignable(name),
            MutFnCtx::Const(ctx) => ctx.is_assignable(name),
            MutFnCtx::Mut(ctx) => ctx.is_assignable(name),
        }
    }

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.put_value(name, value),
            MutFnCtx::Const(ctx) => ctx.put_value(name, value),
            MutFnCtx::Mut(ctx) => ctx.put_value(name, value),
        }
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.set_final(name),
            MutFnCtx::Const(ctx) => ctx.set_final(name),
            MutFnCtx::Mut(ctx) => ctx.set_final(name),
        }
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.is_final(name),
            MutFnCtx::Const(ctx) => ctx.is_final(name),
            MutFnCtx::Mut(ctx) => ctx.is_final(name),
        }
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.set_const(name),
            MutFnCtx::Const(ctx) => ctx.set_const(name),
            MutFnCtx::Mut(ctx) => ctx.set_const(name),
        }
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.is_const(name),
            MutFnCtx::Const(ctx) => ctx.is_const(name),
            MutFnCtx::Mut(ctx) => ctx.is_const(name),
        }
    }
}

impl<'l> CtxRef<'l> for MutFnCtx<'l> {
    fn get_solver(self) -> Result<&'l FuncVal, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_solver(),
            MutFnCtx::Const(ctx) => ctx.get_solver(),
            MutFnCtx::Mut(ctx) => ctx.get_solver(),
        }
    }

    fn get_solver_mut(self) -> Result<&'l mut FuncVal, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_solver_mut(),
            MutFnCtx::Const(ctx) => ctx.get_solver_mut(),
            MutFnCtx::Mut(ctx) => ctx.get_solver_mut(),
        }
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, FuncVal>, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_solver_dyn(),
            MutFnCtx::Const(ctx) => ctx.get_solver_dyn(),
            MutFnCtx::Mut(ctx) => ctx.get_solver_dyn(),
        }
    }

    fn set_solver(self, solver: Option<FuncVal>) -> Result<(), CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.set_solver(solver),
            MutFnCtx::Const(ctx) => ctx.set_solver(solver),
            MutFnCtx::Mut(ctx) => ctx.set_solver(solver),
        }
    }
}

impl<'l> CtxMeta<'l> for MutFnCtx<'l> {
    type Reborrow<'s> = MutFnCtx<'s> where Self: 's;

    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        match self {
            MutFnCtx::Free(_ctx) => MutFnCtx::Free(FreeCtx),
            MutFnCtx::Const(ctx) => MutFnCtx::Const(ctx.reborrow()),
            MutFnCtx::Mut(ctx) => MutFnCtx::Mut(ctx.reborrow()),
        }
    }

    fn borrow(&self) -> Option<&Ctx> {
        match self {
            MutFnCtx::Free(ctx) => ctx.borrow(),
            MutFnCtx::Const(ctx) => ctx.borrow(),
            MutFnCtx::Mut(ctx) => ctx.borrow(),
        }
    }

    fn is_ctx_free(self) -> bool {
        matches!(self, MutFnCtx::Free(_))
    }

    fn is_ctx_const(self) -> bool {
        matches!(self, MutFnCtx::Free(_) | MutFnCtx::Const(_))
    }

    fn for_const_fn(self) -> ConstFnCtx<'l> {
        match self {
            MutFnCtx::Free(_ctx) => ConstFnCtx::Free(FreeCtx),
            MutFnCtx::Const(ctx) => ConstFnCtx::Const(ctx),
            MutFnCtx::Mut(ctx) => ConstFnCtx::Const(ConstCtx::new(ctx.0)),
        }
    }

    fn for_mut_fn(self) -> MutFnCtx<'l> {
        self
    }
}

impl<'a> MutCtx<'a> {
    pub fn new(ctx: &'a mut Ctx) -> Self {
        Self(ctx)
    }

    pub fn reborrow(&mut self) -> MutCtx {
        MutCtx(self.0)
    }

    pub fn borrow(&self) -> Option<&Ctx> {
        <_ as CtxMeta<'a>>::borrow(self)
    }

    pub fn swap(&mut self, other: &mut Self) {
        swap(self.0, other.0);
    }

    pub fn set(&mut self, ctx: Ctx) {
        *self.0 = ctx;
    }

    // INVARIANT: The function f can take the ctx out during its execution,
    // but when f returns, ctx must be equal to its original value.
    pub(crate) fn temp_take<'b, T, F>(&'b mut self, f: F) -> T
    where
        F: FnOnce(&'b mut Ctx) -> T,
    {
        f(self.0)
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        <_ as CtxMapRef>::get_ref(self, name)
    }

    pub fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError> {
        <_ as CtxMapRef>::get_ref_mut(self, name)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_assignable(self, name: Symbol) -> bool {
        <_ as CtxMapRef>::is_assignable(self, name)
    }

    pub fn put(
        self,
        name: Symbol,
        invariant: Invariant,
        val: Val,
    ) -> Result<Option<Val>, CtxError> {
        self.put_value(name, CtxValue { invariant, val })
    }
}

impl<'a> MutFnCtx<'a> {
    pub fn reborrow(&mut self) -> MutFnCtx {
        <_ as CtxMeta<'a>>::reborrow(self)
    }

    pub fn borrow(&self) -> Option<&Ctx> {
        <_ as CtxMeta<'a>>::borrow(self)
    }

    pub fn to_const(self) -> ConstFnCtx<'a> {
        match self {
            MutFnCtx::Free(_ctx) => ConstFnCtx::Free(FreeCtx),
            MutFnCtx::Const(ctx) => ConstFnCtx::Const(ctx),
            MutFnCtx::Mut(ctx) => ConstFnCtx::Const(ConstCtx::new(ctx.0)),
        }
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        <_ as CtxMapRef>::get_ref(self, name)
    }

    pub fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError> {
        <_ as CtxMapRef>::get_ref_mut(self, name)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_assignable(self, name: Symbol) -> bool {
        <_ as CtxMapRef>::is_assignable(self, name)
    }
}
