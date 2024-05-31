use std::matches;

use crate::{
    ctx::{
        mutable::CtxForMutableFn,
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

pub enum CtxForConstFn<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
}

impl<'l> CtxRef<'l> for ConstCtx<'l> {
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

    fn get_meta(self) -> Result<&'l Ctx, CtxError> {
        self.0.get_meta()
    }

    fn get_meta_mut(self) -> Result<&'l mut Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta_dyn(self) -> Result<DynRef<'l, Ctx>, CtxError> {
        let mut dyn_ref = self.0.get_meta_dyn()?;
        dyn_ref.is_const = true;
        Ok(dyn_ref)
    }

    fn set_meta(self, _meta: Option<Ctx>) -> Result<(), CtxError> {
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

    fn for_const_fn(self) -> CtxForConstFn<'l> {
        CtxForConstFn::Const(self)
    }

    fn for_mutable_fn(self) -> CtxForMutableFn<'l> {
        CtxForMutableFn::Const(self)
    }
}

impl<'l> CtxRef<'l> for CtxForConstFn<'l> {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_ref(name),
            CtxForConstFn::Const(ctx) => <_ as CtxRef>::get_ref(ctx, name),
        }
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_ref_mut(name),
            CtxForConstFn::Const(ctx) => ctx.get_ref_mut(name),
        }
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_ref_dyn(name),
            CtxForConstFn::Const(ctx) => ctx.get_ref_dyn(name),
        }
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.remove(name),
            CtxForConstFn::Const(ctx) => ctx.remove(name),
        }
    }

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.put_value(name, value),
            CtxForConstFn::Const(ctx) => ctx.put_value(name, value),
        }
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_final(name),
            CtxForConstFn::Const(ctx) => ctx.set_final(name),
        }
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_final(name),
            CtxForConstFn::Const(ctx) => ctx.is_final(name),
        }
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_const(name),
            CtxForConstFn::Const(ctx) => ctx.set_const(name),
        }
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_const(name),
            CtxForConstFn::Const(ctx) => ctx.is_const(name),
        }
    }

    fn get_meta(self) -> Result<&'l Ctx, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_meta(),
            CtxForConstFn::Const(ctx) => ctx.get_meta(),
        }
    }

    fn get_meta_mut(self) -> Result<&'l mut Ctx, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_meta_mut(),
            CtxForConstFn::Const(ctx) => ctx.get_meta_mut(),
        }
    }

    fn get_meta_dyn(self) -> Result<DynRef<'l, Ctx>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_meta_dyn(),
            CtxForConstFn::Const(ctx) => ctx.get_meta_dyn(),
        }
    }

    fn set_meta(self, meta: Option<Ctx>) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_meta(meta),
            CtxForConstFn::Const(ctx) => ctx.set_meta(meta),
        }
    }
}

impl<'l> CtxMeta<'l> for CtxForConstFn<'l> {
    type Reborrow<'s> = CtxForConstFn<'s> where 'l: 's;

    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        match self {
            CtxForConstFn::Free(ctx) => CtxForConstFn::Free(ctx.reborrow()),
            CtxForConstFn::Const(ctx) => CtxForConstFn::Const(ctx.reborrow()),
        }
    }

    fn borrow(&self) -> Option<&Ctx> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.borrow(),
            CtxForConstFn::Const(ctx) => ctx.borrow(),
        }
    }

    fn is_ctx_free(self) -> bool {
        matches!(self, CtxForConstFn::Free(_))
    }

    fn is_ctx_const(self) -> bool {
        true
    }

    fn for_const_fn(self) -> CtxForConstFn<'l> {
        self
    }

    fn for_mutable_fn(self) -> CtxForMutableFn<'l> {
        match self {
            CtxForConstFn::Free(_ctx) => CtxForMutableFn::Free(FreeCtx),
            CtxForConstFn::Const(ctx) => CtxForMutableFn::Const(ctx),
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

    pub fn meta(self) -> Option<ConstCtx<'a>> {
        self.0.meta.as_mut().map(|meta| ConstCtx(&mut *meta))
    }

    pub(crate) fn get_ctx_ref(self) -> &'a Ctx {
        self.0
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        <_ as CtxRef>::get_ref(self, name)
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
        <_ as CtxMeta<'a>>::reborrow(self)
    }

    pub fn borrow(&self) -> Option<&Ctx> {
        <_ as CtxMeta<'a>>::borrow(self)
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        <_ as CtxRef>::get_ref(self, name)
    }

    pub fn meta(self) -> Result<ConstCtx<'a>, CtxError> {
        match self {
            CtxForConstFn::Free(_ctx) => Err(CtxError::AccessDenied),
            CtxForConstFn::Const(ctx) => match ctx.meta() {
                Some(meta) => Ok(meta),
                None => Err(CtxError::NotFound),
            },
        }
    }
}
