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
            free::FreeCtx,
            mutable::CtxForMutableFn,
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

pub(crate) struct ConstCtx<'a>(pub(crate) &'a mut Ctx);

/*
Why `&mut Ctx`? What we actually need is an owned `Ctx`, because we need to store the ctx when
evaluating a ctx-aware function. But a `&mut Ctx` is more compact and convenient, and we can
change `&mut Ctx` back to `Ctx` at anytime we need by swapping its memory with a default ctx.
The `const` is just a flag and a runtime invariant.
*/
pub(crate) enum CtxForConstFn<'a> {
    Free,
    Const(&'a mut Ctx),
}

impl<'a> CtxTrait for ConstCtx<'a> {
    fn get(&mut self, name: &str) -> Val {
        self.0.get(name)
    }

    fn is_null(&mut self, name: &str) -> Val {
        DefaultCtx.is_null(self, name)
    }

    fn remove(&mut self, name: &str) -> Val {
        self.0.remove(true, name)
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        self.0.put_val(true, name, val)
    }

    fn put_val_local(&mut self, _name: Symbol, _val: TaggedVal) -> Val {
        Val::default()
    }

    fn set_final(&mut self, name: &str) {
        self.0.set_final(true, name);
    }

    fn set_const(&mut self, name: &str) {
        self.0.set_const(true, name);
    }

    fn is_final(&mut self, name: &str) -> Val {
        let is_final = self.0.is_final(name);
        Val::Bool(Bool::new(is_final))
    }

    fn is_const(&mut self, name: &str) -> Val {
        let is_const = self.0.is_const(name);
        Val::Bool(Bool::new(is_const))
    }

    fn set_super(&mut self, _super_ctx: Option<Either<Symbol, RefVal>>) {}

    fn get_ref<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>) -> T,
    {
        self.0.get_ref(true, name, |val, _| f(val))
    }
}

impl<'a> CtxAccessor for ConstCtx<'a> {
    fn for_const_fn(&mut self) -> CtxForConstFn {
        CtxForConstFn::Const(self.0)
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Const(self.0)
    }
}

impl<'a> CtxTrait for CtxForConstFn<'a> {
    fn get(&mut self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free => FreeCtx.get(name),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).get(name),
        }
    }

    fn is_null(&mut self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free => FreeCtx.is_null(name),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).is_null(name),
        }
    }

    fn remove(&mut self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free => FreeCtx.remove(name),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).remove(name),
        }
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForConstFn::Free => FreeCtx.put_val(name, val),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).put_val(name, val),
        }
    }

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForConstFn::Free => FreeCtx.put_val_local(name, val),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).put_val_local(name, val),
        }
    }

    fn set_final(&mut self, name: &str) {
        match self {
            CtxForConstFn::Free => FreeCtx.set_final(name),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).set_final(name),
        }
    }

    fn set_const(&mut self, name: &str) {
        match self {
            CtxForConstFn::Free => FreeCtx.set_const(name),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).set_const(name),
        }
    }

    fn is_final(&mut self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free => FreeCtx.is_final(name),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).is_final(name),
        }
    }

    fn is_const(&mut self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free => FreeCtx.is_const(name),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).is_const(name),
        }
    }

    fn set_super(&mut self, super_ctx: Option<Either<Symbol, RefVal>>) {
        match self {
            CtxForConstFn::Free => FreeCtx.set_super(super_ctx),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).set_super(super_ctx),
        }
    }

    fn get_ref<T, F>(&mut self, name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>) -> T,
    {
        match self {
            CtxForConstFn::Free => FreeCtx.get_ref(name, f),
            CtxForConstFn::Const(ctx) => ConstCtx(ctx).get_ref(name, f),
        }
    }
}

impl<'a> CtxAccessor for CtxForConstFn<'a> {
    fn for_const_fn(&mut self) -> CtxForConstFn {
        self.reborrow()
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        match self {
            CtxForConstFn::Free => CtxForMutableFn::Free,
            CtxForConstFn::Const(ctx) => CtxForMutableFn::Const(ctx),
        }
    }
}

impl<'a> ConstCtx<'a> {
    #[allow(unused)]
    pub(crate) fn reborrow(&mut self) -> ConstCtx {
        ConstCtx(self.0)
    }
}

impl<'a> CtxForConstFn<'a> {
    pub(crate) fn reborrow(&mut self) -> CtxForConstFn {
        match self {
            CtxForConstFn::Free => CtxForConstFn::Free,
            CtxForConstFn::Const(ctx) => CtxForConstFn::Const(ctx),
        }
    }
}