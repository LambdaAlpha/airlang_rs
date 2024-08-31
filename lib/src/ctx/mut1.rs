use std::{
    matches,
    mem::swap,
};

use crate::{
    ctx::{
        map::{
            CtxMap,
            CtxMapRef,
        },
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

impl<'l> CtxRef<'l> for MutCtx<'l> {
    fn get_variables(self) -> Result<&'l CtxMap, CtxError> {
        self.0.get_variables()
    }

    fn get_variables_mut(self) -> Result<&'l mut CtxMap, CtxError> {
        self.0.get_variables_mut()
    }

    fn get_variables_dyn(self) -> Result<DynRef<'l, CtxMap>, CtxError> {
        self.0.get_variables_dyn()
    }

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

    fn is_unchecked(self) -> bool {
        self.0.is_unchecked()
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

impl<'l> CtxRef<'l> for MutFnCtx<'l> {
    fn get_variables(self) -> Result<&'l CtxMap, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_variables(),
            MutFnCtx::Const(ctx) => ctx.get_variables(),
            MutFnCtx::Mut(ctx) => ctx.get_variables(),
        }
    }

    fn get_variables_mut(self) -> Result<&'l mut CtxMap, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_variables_mut(),
            MutFnCtx::Const(ctx) => ctx.get_variables_mut(),
            MutFnCtx::Mut(ctx) => ctx.get_variables_mut(),
        }
    }

    fn get_variables_dyn(self) -> Result<DynRef<'l, CtxMap>, CtxError> {
        match self {
            MutFnCtx::Free(ctx) => ctx.get_variables_dyn(),
            MutFnCtx::Const(ctx) => ctx.get_variables_dyn(),
            MutFnCtx::Mut(ctx) => ctx.get_variables_dyn(),
        }
    }

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

    fn is_unchecked(self) -> bool {
        match self {
            MutFnCtx::Free(ctx) => ctx.is_unchecked(),
            MutFnCtx::Const(ctx) => ctx.is_unchecked(),
            MutFnCtx::Mut(ctx) => ctx.is_unchecked(),
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
        self.get_variables()?.get_ref(name)
    }

    pub fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError> {
        self.get_variables_mut()?.get_ref_mut(name)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_assignable(self, name: Symbol) -> bool {
        let Ok(ctx) = self.get_variables() else {
            return false;
        };
        ctx.is_assignable(name)
    }

    pub fn put(
        self,
        name: Symbol,
        invariant: Invariant,
        val: Val,
    ) -> Result<Option<Val>, CtxError> {
        self.get_variables_mut()?
            .put_value(name, CtxValue { invariant, val })
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
        <_ as CtxMeta>::for_const_fn(self)
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        self.get_variables()?.get_ref(name)
    }

    pub fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError> {
        self.get_variables_mut()?.get_ref_mut(name)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_assignable(self, name: Symbol) -> bool {
        let Ok(ctx) = self.get_variables() else {
            return false;
        };
        ctx.is_assignable(name)
    }
}