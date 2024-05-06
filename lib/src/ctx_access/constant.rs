use crate::{
    ctx::{
        Ctx,
        CtxError,
        CtxTrait,
        CtxValue,
        DynRef,
    },
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    symbol::Symbol,
    Val,
};

/*
Why `&mut Ctx`? What we actually need is an owned `Ctx`, because we need to store the ctx when
evaluating a ctx-aware function. But a `&mut Ctx` is more compact and convenient, and we can
change `&mut Ctx` back to `Ctx` at anytime we need by swapping its memory with a default ctx.
The `const` is just a flag and a runtime invariant.
*/
pub struct ConstCtx<'a>(&'a mut Ctx);

pub enum CtxForConstFn<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
}

impl<'a> CtxTrait for ConstCtx<'a> {
    fn get_ref(&self, name: &Symbol) -> Result<&Val, CtxError> {
        self.0.get_ref(name)
    }

    fn get_ref_mut(&mut self, _name: &Symbol) -> Result<&mut Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_dyn(&mut self, name: &Symbol) -> Result<DynRef<Val>, CtxError> {
        let mut dyn_ref = self.0.get_ref_dyn(name)?;
        dyn_ref.is_const = true;
        Ok(dyn_ref)
    }

    fn remove(&mut self, _name: &Symbol) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn put_value(&mut self, _name: Symbol, _value: CtxValue) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(&mut self, _name: &Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_final(&self, name: &Symbol) -> Result<bool, CtxError> {
        self.0.is_final(name)
    }

    fn set_const(&mut self, _name: &Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_const(&self, name: &Symbol) -> Result<bool, CtxError> {
        self.0.is_const(name)
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        self.0.get_meta()
    }

    fn get_meta_mut(&mut self) -> Result<&mut Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta_dyn(&mut self) -> Result<DynRef<Ctx>, CtxError> {
        let mut dyn_ref = self.0.get_meta_dyn()?;
        dyn_ref.is_const = true;
        Ok(dyn_ref)
    }

    fn set_meta(&mut self, _meta: Option<Ctx>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl<'a> CtxAccessor for ConstCtx<'a> {
    fn is_ctx_free(&self) -> bool {
        false
    }

    fn is_ctx_const(&self) -> bool {
        true
    }

    fn for_const_fn(&mut self) -> CtxForConstFn {
        CtxForConstFn::Const(self.reborrow())
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Const(self.reborrow())
    }
}

impl<'a> CtxTrait for CtxForConstFn<'a> {
    fn get_ref(&self, name: &Symbol) -> Result<&Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_ref(name),
            CtxForConstFn::Const(ctx) => <_ as CtxTrait>::get_ref(ctx, name),
        }
    }

    fn get_ref_mut(&mut self, name: &Symbol) -> Result<&mut Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_ref_mut(name),
            CtxForConstFn::Const(ctx) => ctx.get_ref_mut(name),
        }
    }

    fn get_ref_dyn(&mut self, name: &Symbol) -> Result<DynRef<Val>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_ref_dyn(name),
            CtxForConstFn::Const(ctx) => ctx.get_ref_dyn(name),
        }
    }

    fn remove(&mut self, name: &Symbol) -> Result<Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.remove(name),
            CtxForConstFn::Const(ctx) => ctx.remove(name),
        }
    }

    fn put_value(&mut self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.put_value(name, value),
            CtxForConstFn::Const(ctx) => ctx.put_value(name, value),
        }
    }

    fn set_final(&mut self, name: &Symbol) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_final(name),
            CtxForConstFn::Const(ctx) => ctx.set_final(name),
        }
    }

    fn is_final(&self, name: &Symbol) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_final(name),
            CtxForConstFn::Const(ctx) => ctx.is_final(name),
        }
    }

    fn set_const(&mut self, name: &Symbol) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_const(name),
            CtxForConstFn::Const(ctx) => ctx.set_const(name),
        }
    }

    fn is_const(&self, name: &Symbol) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_const(name),
            CtxForConstFn::Const(ctx) => ctx.is_const(name),
        }
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_meta(),
            CtxForConstFn::Const(ctx) => ctx.get_meta(),
        }
    }

    fn get_meta_mut(&mut self) -> Result<&mut Ctx, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_meta_mut(),
            CtxForConstFn::Const(ctx) => ctx.get_meta_mut(),
        }
    }

    fn get_meta_dyn(&mut self) -> Result<DynRef<Ctx>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_meta_dyn(),
            CtxForConstFn::Const(ctx) => ctx.get_meta_dyn(),
        }
    }

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_meta(meta),
            CtxForConstFn::Const(ctx) => ctx.set_meta(meta),
        }
    }
}

impl<'a> CtxAccessor for CtxForConstFn<'a> {
    fn is_ctx_free(&self) -> bool {
        matches!(self, CtxForConstFn::Free(_))
    }

    fn is_ctx_const(&self) -> bool {
        true
    }

    fn for_const_fn(&mut self) -> CtxForConstFn {
        self.reborrow()
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        match self {
            CtxForConstFn::Free(_ctx) => CtxForMutableFn::Free(FreeCtx),
            CtxForConstFn::Const(ctx) => CtxForMutableFn::Const(ctx.reborrow()),
        }
    }
}

impl<'a> ConstCtx<'a> {
    pub fn new(ctx: &'a mut Ctx) -> Self {
        ConstCtx(ctx)
    }

    pub fn reborrow(&mut self) -> ConstCtx {
        ConstCtx(self.0)
    }

    pub fn meta(&mut self) -> Option<ConstCtx> {
        self.0.meta.as_mut().map(|meta| ConstCtx(&mut *meta))
    }

    pub(crate) fn get_ctx_ref(&self) -> &Ctx {
        self.0
    }

    pub fn get_ref(&self, name: &Symbol) -> Result<&Val, CtxError> {
        <_ as CtxTrait>::get_ref(self, name)
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

impl<'a> CtxForConstFn<'a> {
    pub fn reborrow(&mut self) -> CtxForConstFn {
        match self {
            CtxForConstFn::Free(_ctx) => CtxForConstFn::Free(FreeCtx),
            CtxForConstFn::Const(ctx) => CtxForConstFn::Const(ctx.reborrow()),
        }
    }

    pub fn get_ref(&self, name: &Symbol) -> Result<&Val, CtxError> {
        <_ as CtxTrait>::get_ref(self, name)
    }

    pub fn meta(&mut self) -> Result<ConstCtx, CtxError> {
        match self {
            CtxForConstFn::Free(_ctx) => Err(CtxError::AccessDenied),
            CtxForConstFn::Const(ctx) => match ctx.meta() {
                Some(meta) => Ok(meta),
                None => Err(CtxError::NotFound),
            },
        }
    }
}
