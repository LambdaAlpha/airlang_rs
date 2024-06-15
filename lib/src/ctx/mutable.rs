use std::{
    matches,
    mem::swap,
};

use crate::{
    ctx::{
        ref1::{
            CtxMeta,
            CtxRef,
        },
        CtxValue,
        DynRef,
    },
    ConstCtx,
    Ctx,
    CtxError,
    CtxForConstFn,
    FreeCtx,
    Invariant,
    Symbol,
    Val,
};

pub struct MutableCtx<'a>(&'a mut Ctx);

pub enum CtxForMutableFn<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
    Mutable(MutableCtx<'a>),
}

impl<'l> CtxRef<'l> for MutableCtx<'l> {
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

    fn get_meta(self) -> Result<&'l Ctx, CtxError> {
        self.0.get_meta()
    }

    fn get_meta_mut(self) -> Result<&'l mut Ctx, CtxError> {
        self.0.get_meta_mut()
    }

    fn get_meta_dyn(self) -> Result<DynRef<'l, Ctx>, CtxError> {
        self.0.get_meta_dyn()
    }

    fn set_meta(self, meta: Option<Ctx>) -> Result<(), CtxError> {
        self.0.set_meta(meta)
    }
}

impl<'l> CtxMeta<'l> for MutableCtx<'l> {
    type Reborrow<'s> = MutableCtx<'s> where Self: 's;
    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        MutableCtx(self.0)
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
    fn for_const_fn(self) -> CtxForConstFn<'l> {
        CtxForConstFn::Const(ConstCtx::new(self.0))
    }
    fn for_mutable_fn(self) -> CtxForMutableFn<'l> {
        CtxForMutableFn::Mutable(self)
    }
}

impl<'l> CtxRef<'l> for CtxForMutableFn<'l> {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_ref(name),
            CtxForMutableFn::Const(ctx) => <_ as CtxRef>::get_ref(ctx, name),
            CtxForMutableFn::Mutable(ctx) => <_ as CtxRef>::get_ref(ctx, name),
        }
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_ref_mut(name),
            CtxForMutableFn::Const(ctx) => ctx.get_ref_mut(name),
            CtxForMutableFn::Mutable(ctx) => <_ as CtxRef>::get_ref_mut(ctx, name),
        }
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_ref_dyn(name),
            CtxForMutableFn::Const(ctx) => ctx.get_ref_dyn(name),
            CtxForMutableFn::Mutable(ctx) => ctx.get_ref_dyn(name),
        }
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.remove(name),
            CtxForMutableFn::Const(ctx) => ctx.remove(name),
            CtxForMutableFn::Mutable(ctx) => ctx.remove(name),
        }
    }

    fn is_assignable(self, name: Symbol) -> bool {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_assignable(name),
            CtxForMutableFn::Const(ctx) => ctx.is_assignable(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_assignable(name),
        }
    }

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.put_value(name, value),
            CtxForMutableFn::Const(ctx) => ctx.put_value(name, value),
            CtxForMutableFn::Mutable(ctx) => ctx.put_value(name, value),
        }
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_final(name),
            CtxForMutableFn::Const(ctx) => ctx.set_final(name),
            CtxForMutableFn::Mutable(ctx) => ctx.set_final(name),
        }
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_final(name),
            CtxForMutableFn::Const(ctx) => ctx.is_final(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_final(name),
        }
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_const(name),
            CtxForMutableFn::Const(ctx) => ctx.set_const(name),
            CtxForMutableFn::Mutable(ctx) => ctx.set_const(name),
        }
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_const(name),
            CtxForMutableFn::Const(ctx) => ctx.is_const(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_const(name),
        }
    }

    fn get_meta(self) -> Result<&'l Ctx, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_meta(),
            CtxForMutableFn::Const(ctx) => ctx.get_meta(),
            CtxForMutableFn::Mutable(ctx) => ctx.get_meta(),
        }
    }

    fn get_meta_mut(self) -> Result<&'l mut Ctx, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_meta_mut(),
            CtxForMutableFn::Const(ctx) => ctx.get_meta_mut(),
            CtxForMutableFn::Mutable(ctx) => ctx.get_meta_mut(),
        }
    }

    fn get_meta_dyn(self) -> Result<DynRef<'l, Ctx>, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_meta_dyn(),
            CtxForMutableFn::Const(ctx) => ctx.get_meta_dyn(),
            CtxForMutableFn::Mutable(ctx) => ctx.get_meta_dyn(),
        }
    }

    fn set_meta(self, meta: Option<Ctx>) -> Result<(), CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_meta(meta),
            CtxForMutableFn::Const(ctx) => ctx.set_meta(meta),
            CtxForMutableFn::Mutable(ctx) => <_ as CtxRef>::set_meta(ctx, meta),
        }
    }
}

impl<'l> CtxMeta<'l> for CtxForMutableFn<'l> {
    type Reborrow<'s> = CtxForMutableFn<'s> where Self: 's;

    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        match self {
            CtxForMutableFn::Free(_ctx) => CtxForMutableFn::Free(FreeCtx),
            CtxForMutableFn::Const(ctx) => CtxForMutableFn::Const(ctx.reborrow()),
            CtxForMutableFn::Mutable(ctx) => CtxForMutableFn::Mutable(ctx.reborrow()),
        }
    }

    fn borrow(&self) -> Option<&Ctx> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.borrow(),
            CtxForMutableFn::Const(ctx) => ctx.borrow(),
            CtxForMutableFn::Mutable(ctx) => ctx.borrow(),
        }
    }

    fn is_ctx_free(self) -> bool {
        matches!(self, CtxForMutableFn::Free(_))
    }

    fn is_ctx_const(self) -> bool {
        matches!(self, CtxForMutableFn::Free(_) | CtxForMutableFn::Const(_))
    }

    fn for_const_fn(self) -> CtxForConstFn<'l> {
        match self {
            CtxForMutableFn::Free(_ctx) => CtxForConstFn::Free(FreeCtx),
            CtxForMutableFn::Const(ctx) => CtxForConstFn::Const(ctx),
            CtxForMutableFn::Mutable(ctx) => CtxForConstFn::Const(ConstCtx::new(ctx.0)),
        }
    }

    fn for_mutable_fn(self) -> CtxForMutableFn<'l> {
        self
    }
}

impl<'a> MutableCtx<'a> {
    pub fn new(ctx: &'a mut Ctx) -> Self {
        Self(ctx)
    }

    pub fn reborrow(&mut self) -> MutableCtx {
        MutableCtx(self.0)
    }

    pub fn borrow(&self) -> Option<&Ctx> {
        <_ as CtxMeta<'a>>::borrow(self)
    }

    pub fn meta(self) -> Option<MutableCtx<'a>> {
        self.0.meta.as_mut().map(|meta| MutableCtx(&mut *meta))
    }

    pub fn set_meta(self, meta: Option<Ctx>) {
        self.0.meta = meta.map(Box::new);
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
        <_ as CtxRef>::get_ref(self, name)
    }

    pub fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError> {
        <_ as CtxRef>::get_ref_mut(self, name)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_assignable(self, name: Symbol) -> bool {
        <_ as CtxRef>::is_assignable(self, name)
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

impl<'a> CtxForMutableFn<'a> {
    pub fn reborrow(&mut self) -> CtxForMutableFn {
        <_ as CtxMeta<'a>>::reborrow(self)
    }

    pub fn borrow(&self) -> Option<&Ctx> {
        <_ as CtxMeta<'a>>::borrow(self)
    }

    pub fn to_const(self) -> CtxForConstFn<'a> {
        match self {
            CtxForMutableFn::Free(_ctx) => CtxForConstFn::Free(FreeCtx),
            CtxForMutableFn::Const(ctx) => CtxForConstFn::Const(ctx),
            CtxForMutableFn::Mutable(ctx) => CtxForConstFn::Const(ConstCtx::new(ctx.0)),
        }
    }

    pub fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError> {
        <_ as CtxRef>::get_ref(self, name)
    }

    pub fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError> {
        <_ as CtxRef>::get_ref_mut(self, name)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_assignable(self, name: Symbol) -> bool {
        <_ as CtxRef>::is_assignable(self, name)
    }

    pub fn meta(self) -> Result<ConstCtx<'a>, CtxError> {
        match self {
            CtxForMutableFn::Free(_ctx) => Err(CtxError::AccessDenied),
            CtxForMutableFn::Const(ctx) => match ctx.meta() {
                Some(meta) => Ok(meta),
                None => Err(CtxError::NotFound),
            },
            CtxForMutableFn::Mutable(ctx) => match ctx.meta() {
                Some(meta) => Ok(ConstCtx::new(meta.0)),
                None => Err(CtxError::NotFound),
            },
        }
    }

    pub fn meta_mut(self) -> Result<MutableCtx<'a>, CtxError> {
        match self {
            CtxForMutableFn::Free(_ctx) => Err(CtxError::AccessDenied),
            CtxForMutableFn::Const(_ctx) => Err(CtxError::AccessDenied),
            CtxForMutableFn::Mutable(ctx) => match ctx.meta() {
                Some(meta) => Ok(meta),
                None => Err(CtxError::NotFound),
            },
        }
    }
}
