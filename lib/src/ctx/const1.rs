use std::matches;

use crate::{
    Ctx,
    CtxError,
    FreeCtx,
    Symbol,
    Val,
    ctx::{
        DynRef,
        map::{
            CtxMap,
            CtxMapRef,
        },
        mut1::MutFnCtx,
        ref1::{
            CtxMeta,
            CtxRef,
        },
    },
    val::func::CellFuncVal,
};
/*
Why `&mut Ctx`? What we actually need is an owned `Ctx`, because we need to store the ctx when
evaluating a ctx-aware function. But a `&mut Ctx` is more compact and convenient, and we can
change `&mut Ctx` back to `Ctx` at any time we need by swapping its memory with a default ctx.
The `const` is just a flag and a runtime invariant.
*/
pub struct ConstCtx<'a>(&'a mut Ctx);

pub enum ConstFnCtx<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
}

impl<'l> CtxRef<'l> for ConstCtx<'l> {
    fn get_variables(self) -> Result<&'l CtxMap, CtxError> {
        self.0.get_variables()
    }

    fn get_variables_mut(self) -> Result<&'l mut CtxMap, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_variables_dyn(self) -> Result<DynRef<'l, CtxMap>, CtxError> {
        let mut dyn_ref = self.0.get_variables_dyn()?;
        dyn_ref.is_const = true;
        Ok(dyn_ref)
    }

    fn get_solver(self) -> Result<&'l CellFuncVal, CtxError> {
        self.0.get_solver()
    }

    fn get_solver_mut(self) -> Result<&'l mut CellFuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, CellFuncVal>, CtxError> {
        let mut dyn_ref = self.0.get_solver_dyn()?;
        dyn_ref.is_const = true;
        Ok(dyn_ref)
    }

    fn set_solver(self, _solver: Option<CellFuncVal>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl<'l> CtxMeta<'l> for ConstCtx<'l> {
    type Reborrow<'s>
        = ConstCtx<'s>
    where
        Self: 's;

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

impl<'l> CtxRef<'l> for ConstFnCtx<'l> {
    fn get_variables(self) -> Result<&'l CtxMap, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_variables(),
            ConstFnCtx::Const(ctx) => ctx.get_variables(),
        }
    }

    fn get_variables_mut(self) -> Result<&'l mut CtxMap, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_variables_mut(),
            ConstFnCtx::Const(ctx) => ctx.get_variables_mut(),
        }
    }

    fn get_variables_dyn(self) -> Result<DynRef<'l, CtxMap>, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_variables_dyn(),
            ConstFnCtx::Const(ctx) => ctx.get_variables_dyn(),
        }
    }

    fn get_solver(self) -> Result<&'l CellFuncVal, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_solver(),
            ConstFnCtx::Const(ctx) => ctx.get_solver(),
        }
    }

    fn get_solver_mut(self) -> Result<&'l mut CellFuncVal, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_solver_mut(),
            ConstFnCtx::Const(ctx) => ctx.get_solver_mut(),
        }
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, CellFuncVal>, CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.get_solver_dyn(),
            ConstFnCtx::Const(ctx) => ctx.get_solver_dyn(),
        }
    }

    fn set_solver(self, solver: Option<CellFuncVal>) -> Result<(), CtxError> {
        match self {
            ConstFnCtx::Free(ctx) => ctx.set_solver(solver),
            ConstFnCtx::Const(ctx) => ctx.set_solver(solver),
        }
    }
}

impl<'l> CtxMeta<'l> for ConstFnCtx<'l> {
    type Reborrow<'s>
        = ConstFnCtx<'s>
    where
        'l: 's;

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
        self.get_variables()?.get_ref(name)
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
        self.get_variables()?.get_ref(name)
    }
}
