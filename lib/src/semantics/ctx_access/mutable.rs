use crate::{
    semantics::{
        ctx::{
            Ctx,
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
        val::RefVal,
        Val,
    },
    types::{
        Bool,
        Either,
        Symbol,
    },
};

pub(crate) struct MutableCtx<'a>(pub(crate) &'a mut Ctx);

pub(crate) enum CtxForMutableFn<'a> {
    Free,
    Const(&'a mut Ctx),
    Mutable(&'a mut Ctx),
}

impl<'a> CtxTrait for MutableCtx<'a> {
    fn get(&mut self, name: &str) -> Val {
        self.0.get(name)
    }

    fn is_null(&mut self, name: &str) -> Val {
        DefaultCtx.is_null(self, name)
    }

    fn remove(&mut self, name: &str) -> Val {
        self.0.remove(false, name)
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        self.0.put_val(false, name, val)
    }

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val {
        self.0.put_val_local(name, val)
    }

    fn set_final(&mut self, name: &str) {
        self.0.set_final(false, name);
    }

    fn set_const(&mut self, name: &str) {
        self.0.set_const(false, name);
    }

    fn is_final(&mut self, name: &str) -> Val {
        let is_final = self.0.is_final(name);
        Val::Bool(Bool::new(is_final))
    }

    fn is_const(&mut self, name: &str) -> Val {
        let is_const = self.0.is_const(name);
        Val::Bool(Bool::new(is_const))
    }

    fn set_super(&mut self, super_ctx: Option<Either<Symbol, RefVal>>) {
        self.0.super_ctx = super_ctx;
    }

    fn get_ref<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>) -> T,
    {
        self.0.get_ref(false, name, |val, _| f(val))
    }
}

impl<'a> CtxAccessor for MutableCtx<'a> {
    fn for_const_fn(&mut self) -> CtxForConstFn {
        CtxForConstFn::Const(self.0)
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Mutable(self.0)
    }
}

impl<'a> CtxTrait for CtxForMutableFn<'a> {
    fn get(&mut self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free => FreeCtx.get(name),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).get(name),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).get(name),
        }
    }

    fn is_null(&mut self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free => FreeCtx.is_null(name),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).is_null(name),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).is_null(name),
        }
    }

    fn remove(&mut self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free => FreeCtx.remove(name),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).remove(name),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).remove(name),
        }
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForMutableFn::Free => FreeCtx.put_val(name, val),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).put_val(name, val),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).put_val(name, val),
        }
    }

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForMutableFn::Free => FreeCtx.put_val_local(name, val),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).put_val_local(name, val),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).put_val_local(name, val),
        }
    }

    fn set_final(&mut self, name: &str) {
        match self {
            CtxForMutableFn::Free => FreeCtx.set_final(name),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).set_final(name),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).set_final(name),
        }
    }

    fn set_const(&mut self, name: &str) {
        match self {
            CtxForMutableFn::Free => FreeCtx.set_const(name),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).set_const(name),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).set_const(name),
        }
    }

    fn is_final(&mut self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free => FreeCtx.is_final(name),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).is_final(name),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).is_final(name),
        }
    }

    fn is_const(&mut self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free => FreeCtx.is_const(name),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).is_const(name),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).is_const(name),
        }
    }

    fn set_super(&mut self, super_ctx: Option<Either<Symbol, RefVal>>) {
        match self {
            CtxForMutableFn::Free => FreeCtx.set_super(super_ctx),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).set_super(super_ctx),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).set_super(super_ctx),
        }
    }

    fn get_ref<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>) -> T,
    {
        match self {
            CtxForMutableFn::Free => FreeCtx.get_ref(name, f),
            CtxForMutableFn::Const(ctx) => ConstCtx(ctx).get_ref(name, f),
            CtxForMutableFn::Mutable(ctx) => MutableCtx(ctx).get_ref(name, f),
        }
    }
}

impl<'a> CtxAccessor for CtxForMutableFn<'a> {
    fn for_const_fn(&mut self) -> CtxForConstFn {
        match self {
            CtxForMutableFn::Free => CtxForConstFn::Free,
            CtxForMutableFn::Const(ctx) => CtxForConstFn::Const(ctx),
            CtxForMutableFn::Mutable(ctx) => CtxForConstFn::Const(ctx),
        }
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        self.reborrow()
    }
}

impl<'a> MutableCtx<'a> {
    #[allow(unused)]
    pub(crate) fn reborrow(&mut self) -> MutableCtx {
        MutableCtx(self.0)
    }
}

impl<'a> CtxForMutableFn<'a> {
    pub(crate) fn reborrow(&mut self) -> CtxForMutableFn {
        match self {
            CtxForMutableFn::Free => CtxForMutableFn::Free,
            CtxForMutableFn::Const(ctx) => CtxForMutableFn::Const(ctx),
            CtxForMutableFn::Mutable(ctx) => CtxForMutableFn::Mutable(ctx),
        }
    }
}
