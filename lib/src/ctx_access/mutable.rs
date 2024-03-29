use std::mem::swap;

use crate::{
    ctx::{
        Ctx,
        CtxError,
        CtxTrait,
        DefaultCtx,
        InvariantTag,
        TaggedRef,
        TaggedVal,
    },
    ctx_access::{
        constant::{
            ConstCtx,
            CtxForConstFn,
        },
        free::FreeCtx,
        CtxAccessor,
    },
    symbol::Symbol,
    Val,
};

pub struct MutableCtx<'a>(&'a mut Ctx);

pub enum CtxForMutableFn<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
    Mutable(MutableCtx<'a>),
}

impl<'a> CtxTrait for MutableCtx<'a> {
    fn get(&self, name: &str) -> Result<Val, CtxError> {
        self.0.get(name)
    }

    fn remove(&mut self, name: &str) -> Result<Val, CtxError> {
        self.0.remove(name)
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError> {
        self.0.put_val(name, val)
    }

    fn set_final(&mut self, name: &str) -> Result<(), CtxError> {
        self.0.set_final(name)
    }

    fn set_const(&mut self, name: &str) -> Result<(), CtxError> {
        self.0.set_const(name)
    }

    fn is_final(&self, name: &str) -> Result<bool, CtxError> {
        self.0.is_final(name)
    }

    fn is_const(&self, name: &str) -> Result<bool, CtxError> {
        self.0.is_const(name)
    }

    fn is_null(&self, name: &str) -> Result<bool, CtxError> {
        DefaultCtx.is_null(self, name)
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        match &self.0.meta {
            Some(ctx) => Ok(ctx),
            None => Err(CtxError::NotFound),
        }
    }

    fn get_tagged_meta(&mut self) -> Result<TaggedRef<Ctx>, CtxError> {
        match &mut self.0.meta {
            Some(ctx) => Ok(TaggedRef::new(ctx, false)),
            None => Err(CtxError::NotFound),
        }
    }

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError> {
        self.0.meta = meta.map(Box::new);
        Ok(())
    }

    fn get_tagged_ref(&mut self, name: &str) -> Result<TaggedRef<Val>, CtxError> {
        self.0.get_tagged_ref(false, name)
    }

    fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError> {
        self.0.get_const_ref(name)
    }
}

impl<'a> CtxAccessor for MutableCtx<'a> {
    fn is_ctx_free(&self) -> bool {
        false
    }

    fn is_ctx_const(&self) -> bool {
        false
    }

    fn for_const_fn(&mut self) -> CtxForConstFn {
        CtxForConstFn::Const(ConstCtx::new(self.0))
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Mutable(self.reborrow())
    }
}

impl<'a> CtxTrait for CtxForMutableFn<'a> {
    fn get(&self, name: &str) -> Result<Val, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get(name),
            CtxForMutableFn::Const(ctx) => ctx.get(name),
            CtxForMutableFn::Mutable(ctx) => ctx.get(name),
        }
    }

    fn remove(&mut self, name: &str) -> Result<Val, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.remove(name),
            CtxForMutableFn::Const(ctx) => ctx.remove(name),
            CtxForMutableFn::Mutable(ctx) => ctx.remove(name),
        }
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.put_val(name, val),
            CtxForMutableFn::Const(ctx) => ctx.put_val(name, val),
            CtxForMutableFn::Mutable(ctx) => ctx.put_val(name, val),
        }
    }

    fn set_final(&mut self, name: &str) -> Result<(), CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_final(name),
            CtxForMutableFn::Const(ctx) => ctx.set_final(name),
            CtxForMutableFn::Mutable(ctx) => ctx.set_final(name),
        }
    }

    fn set_const(&mut self, name: &str) -> Result<(), CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_const(name),
            CtxForMutableFn::Const(ctx) => ctx.set_const(name),
            CtxForMutableFn::Mutable(ctx) => ctx.set_const(name),
        }
    }

    fn is_final(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_final(name),
            CtxForMutableFn::Const(ctx) => ctx.is_final(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_final(name),
        }
    }

    fn is_const(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_const(name),
            CtxForMutableFn::Const(ctx) => ctx.is_const(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_const(name),
        }
    }

    fn is_null(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_null(name),
            CtxForMutableFn::Const(ctx) => ctx.is_null(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_null(name),
        }
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_meta(),
            CtxForMutableFn::Const(ctx) => ctx.get_meta(),
            CtxForMutableFn::Mutable(ctx) => ctx.get_meta(),
        }
    }

    fn get_tagged_meta(&mut self) -> Result<TaggedRef<Ctx>, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_tagged_meta(),
            CtxForMutableFn::Const(ctx) => ctx.get_tagged_meta(),
            CtxForMutableFn::Mutable(ctx) => ctx.get_tagged_meta(),
        }
    }

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_meta(meta),
            CtxForMutableFn::Const(ctx) => ctx.set_meta(meta),
            CtxForMutableFn::Mutable(ctx) => <_ as CtxTrait>::set_meta(ctx, meta),
        }
    }

    fn get_tagged_ref(&mut self, name: &str) -> Result<TaggedRef<Val>, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_tagged_ref(name),
            CtxForMutableFn::Const(ctx) => ctx.get_tagged_ref(name),
            CtxForMutableFn::Mutable(ctx) => ctx.get_tagged_ref(name),
        }
    }

    fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_const_ref(name),
            CtxForMutableFn::Const(ctx) => ctx.get_const_ref(name),
            CtxForMutableFn::Mutable(ctx) => ctx.get_const_ref(name),
        }
    }
}

impl<'a> CtxAccessor for CtxForMutableFn<'a> {
    fn is_ctx_free(&self) -> bool {
        matches!(self, CtxForMutableFn::Free(_))
    }

    fn is_ctx_const(&self) -> bool {
        matches!(self, CtxForMutableFn::Free(_) | CtxForMutableFn::Const(_))
    }

    fn for_const_fn(&mut self) -> CtxForConstFn {
        match self {
            CtxForMutableFn::Free(_ctx) => CtxForConstFn::Free(FreeCtx),
            CtxForMutableFn::Const(ctx) => CtxForConstFn::Const(ctx.reborrow()),
            CtxForMutableFn::Mutable(ctx) => CtxForConstFn::Const(ConstCtx::new(ctx.0)),
        }
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        self.reborrow()
    }
}

impl<'a> MutableCtx<'a> {
    pub fn new(ctx: &'a mut Ctx) -> Self {
        Self(ctx)
    }

    pub fn reborrow(&mut self) -> MutableCtx {
        MutableCtx(self.0)
    }

    pub fn meta(&mut self) -> Option<MutableCtx> {
        self.0.meta.as_mut().map(|meta| MutableCtx(&mut *meta))
    }

    pub fn set_meta(&mut self, meta: Option<Ctx>) {
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

    pub fn get_ref(&self, name: &Symbol) -> Result<&Val, CtxError> {
        self.get_const_ref(name)
    }

    pub fn get_mut(&mut self, name: &Symbol) -> Result<&mut Val, CtxError> {
        let tagged_ref = self.get_tagged_ref(name)?;
        if tagged_ref.is_const {
            return Err(CtxError::AccessDenied);
        }
        Ok(tagged_ref.val_ref)
    }

    pub fn put(
        &mut self,
        name: Symbol,
        tag: InvariantTag,
        val: Val,
    ) -> Result<Option<Val>, CtxError> {
        self.put_val(name, TaggedVal { tag, val })
    }
}

impl<'a> CtxForMutableFn<'a> {
    pub fn reborrow(&mut self) -> CtxForMutableFn {
        match self {
            CtxForMutableFn::Free(_ctx) => CtxForMutableFn::Free(FreeCtx),
            CtxForMutableFn::Const(ctx) => CtxForMutableFn::Const(ctx.reborrow()),
            CtxForMutableFn::Mutable(ctx) => CtxForMutableFn::Mutable(ctx.reborrow()),
        }
    }

    pub fn to_const(self) -> CtxForConstFn<'a> {
        match self {
            CtxForMutableFn::Free(_ctx) => CtxForConstFn::Free(FreeCtx),
            CtxForMutableFn::Const(ctx) => CtxForConstFn::Const(ctx),
            CtxForMutableFn::Mutable(ctx) => CtxForConstFn::Const(ConstCtx::new(ctx.0)),
        }
    }

    pub fn get_ref(&self, name: &Symbol) -> Result<&Val, CtxError> {
        self.get_const_ref(name)
    }

    pub fn get_mut(&mut self, name: &Symbol) -> Result<&mut Val, CtxError> {
        let tagged_ref = self.get_tagged_ref(name)?;
        if tagged_ref.is_const {
            return Err(CtxError::AccessDenied);
        }
        Ok(tagged_ref.val_ref)
    }

    pub fn meta(&mut self) -> Result<ConstCtx, CtxError> {
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

    pub fn meta_mut(&mut self) -> Result<MutableCtx, CtxError> {
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
