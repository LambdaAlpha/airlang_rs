use crate::{
    ctx::{
        Ctx,
        CtxError,
        CtxTrait,
        DefaultCtx,
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

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError> {
        self.0.put_val_local(name, val)
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

    fn is_local(&self, name: &str) -> Result<bool, CtxError> {
        let is_local = self.0.is_local(name);
        Ok(is_local)
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

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Result<&Val, CtxError>; N] {
        self.0.get_many_const_ref(names)
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
        CtxForConstFn::Const(ConstCtx::new_inner(self.0))
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

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.put_val_local(name, val),
            CtxForMutableFn::Const(ctx) => ctx.put_val_local(name, val),
            CtxForMutableFn::Mutable(ctx) => ctx.put_val_local(name, val),
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

    fn is_local(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_local(name),
            CtxForMutableFn::Const(ctx) => ctx.is_local(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_local(name),
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
            CtxForMutableFn::Mutable(ctx) => ctx.set_meta(meta),
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

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Result<&Val, CtxError>; N] {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_many_const_ref(names),
            CtxForMutableFn::Const(ctx) => ctx.get_many_const_ref(names),
            CtxForMutableFn::Mutable(ctx) => ctx.get_many_const_ref(names),
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
            CtxForMutableFn::Mutable(ctx) => CtxForConstFn::Const(ConstCtx::new_inner(ctx.0)),
        }
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        self.reborrow()
    }
}

impl<'a> MutableCtx<'a> {
    pub fn new(ctx: &'a mut crate::Ctx) -> Self {
        Self(&mut ctx.0)
    }

    pub(crate) fn new_inner(ctx: &'a mut Ctx) -> Self {
        Self(ctx)
    }

    pub fn reborrow(&mut self) -> MutableCtx {
        MutableCtx(self.0)
    }

    // SAFETY: The function f can take the ctx out during its execution,
    // but when f returns, ctx must be equal to its original value.
    pub(crate) unsafe fn temp_take<'b, T, F>(&'b mut self, f: F) -> T
    where
        F: FnOnce(&'b mut Ctx) -> T,
    {
        f(self.0)
    }
}

impl<'a> CtxForMutableFn<'a> {
    pub(crate) fn reborrow(&mut self) -> CtxForMutableFn {
        match self {
            CtxForMutableFn::Free(_ctx) => CtxForMutableFn::Free(FreeCtx),
            CtxForMutableFn::Const(ctx) => CtxForMutableFn::Const(ctx.reborrow()),
            CtxForMutableFn::Mutable(ctx) => CtxForMutableFn::Mutable(ctx.reborrow()),
        }
    }
}
