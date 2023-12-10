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
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    types::Symbol,
    Val,
};

pub struct ConstCtx<'a>(pub(crate) &'a mut Ctx);

/*
Why `&mut Ctx`? What we actually need is an owned `Ctx`, because we need to store the ctx when
evaluating a ctx-aware function. But a `&mut Ctx` is more compact and convenient, and we can
change `&mut Ctx` back to `Ctx` at anytime we need by swapping its memory with a default ctx.
The `const` is just a flag and a runtime invariant.
*/
pub(crate) enum CtxForConstFn<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
}

impl<'a> CtxTrait for ConstCtx<'a> {
    fn get(&self, name: &str) -> Result<Val, CtxError> {
        self.0.get(name)
    }

    fn remove(&mut self, _name: &str) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn put_val(&mut self, _name: Symbol, _val: TaggedVal) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn put_val_local(&mut self, _name: Symbol, _val: TaggedVal) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(&mut self, _name: &str) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_const(&mut self, _name: &str) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
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
            Some(ctx) => Ok(TaggedRef::new(ctx, true)),
            None => Err(CtxError::NotFound),
        }
    }

    fn set_meta(&mut self, _meta: Option<Ctx>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_tagged_ref(&mut self, name: &str) -> Result<TaggedRef<Val>, CtxError> {
        self.0.get_tagged_ref(true, name)
    }

    fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError> {
        self.0.get_const_ref(name)
    }

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Result<&Val, CtxError>; N] {
        self.0.get_many_const_ref(names)
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
    fn get(&self, name: &str) -> Result<Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get(name),
            CtxForConstFn::Const(ctx) => ctx.get(name),
        }
    }

    fn remove(&mut self, name: &str) -> Result<Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.remove(name),
            CtxForConstFn::Const(ctx) => ctx.remove(name),
        }
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.put_val(name, val),
            CtxForConstFn::Const(ctx) => ctx.put_val(name, val),
        }
    }

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Result<Option<Val>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.put_val_local(name, val),
            CtxForConstFn::Const(ctx) => ctx.put_val_local(name, val),
        }
    }

    fn set_final(&mut self, name: &str) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_final(name),
            CtxForConstFn::Const(ctx) => ctx.set_final(name),
        }
    }

    fn set_const(&mut self, name: &str) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_const(name),
            CtxForConstFn::Const(ctx) => ctx.set_const(name),
        }
    }

    fn is_final(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_final(name),
            CtxForConstFn::Const(ctx) => ctx.is_final(name),
        }
    }

    fn is_const(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_const(name),
            CtxForConstFn::Const(ctx) => ctx.is_const(name),
        }
    }

    fn is_null(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_null(name),
            CtxForConstFn::Const(ctx) => ctx.is_null(name),
        }
    }

    fn is_local(&self, name: &str) -> Result<bool, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_local(name),
            CtxForConstFn::Const(ctx) => ctx.is_local(name),
        }
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_meta(),
            CtxForConstFn::Const(ctx) => ctx.get_meta(),
        }
    }

    fn get_tagged_meta(&mut self) -> Result<TaggedRef<Ctx>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_tagged_meta(),
            CtxForConstFn::Const(ctx) => ctx.get_tagged_meta(),
        }
    }

    fn set_meta(&mut self, meta: Option<Ctx>) -> Result<(), CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_meta(meta),
            CtxForConstFn::Const(ctx) => ctx.set_meta(meta),
        }
    }

    fn get_tagged_ref(&mut self, name: &str) -> Result<TaggedRef<Val>, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_tagged_ref(name),
            CtxForConstFn::Const(ctx) => ctx.get_tagged_ref(name),
        }
    }

    fn get_const_ref(&self, name: &str) -> Result<&Val, CtxError> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_const_ref(name),
            CtxForConstFn::Const(ctx) => ctx.get_const_ref(name),
        }
    }

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Result<&Val, CtxError>; N] {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_many_const_ref(names),
            CtxForConstFn::Const(ctx) => ctx.get_many_const_ref(names),
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
    pub(crate) fn reborrow(&mut self) -> ConstCtx {
        ConstCtx(self.0)
    }
}

impl<'a> CtxForConstFn<'a> {
    pub(crate) fn reborrow(&mut self) -> CtxForConstFn {
        match self {
            CtxForConstFn::Free(_ctx) => CtxForConstFn::Free(FreeCtx),
            CtxForConstFn::Const(ctx) => CtxForConstFn::Const(ctx.reborrow()),
        }
    }
}
