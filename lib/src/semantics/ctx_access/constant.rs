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
        Val,
    },
    types::{
        Bool,
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
    Free(FreeCtx),
    Const(ConstCtx<'a>),
}

impl<'a> CtxTrait for ConstCtx<'a> {
    fn get(&self, name: &str) -> Val {
        self.0.get(name)
    }

    fn remove(&mut self, _name: &str) -> Val {
        Val::default()
    }

    fn put_val(&mut self, _name: Symbol, _val: TaggedVal) -> Val {
        Val::default()
    }

    fn put_val_local(&mut self, _name: Symbol, _val: TaggedVal) -> Val {
        Val::default()
    }

    fn set_final(&mut self, _name: &str) {}

    fn set_const(&mut self, _name: &str) {}

    fn is_final(&self, name: &str) -> Val {
        let is_final = self.0.is_final(name);
        Val::Bool(Bool::new(is_final))
    }

    fn is_const(&self, name: &str) -> Val {
        let is_const = self.0.is_const(name);
        Val::Bool(Bool::new(is_const))
    }

    fn is_null(&self, name: &str) -> Val {
        DefaultCtx.is_null(self, name)
    }

    fn is_local(&self, name: &str) -> Val {
        let is_local = self.0.is_local(name);
        Val::Bool(Bool::new(is_local))
    }

    fn set_super(&mut self, _super_ctx: Option<Symbol>) {}

    fn get_tagged_ref(&mut self, name: &str) -> Option<TaggedRef<Val>> {
        self.0.get_tagged_ref(true, name)
    }

    fn get_const_ref(&self, name: &str) -> Option<&Val> {
        self.0.get_const_ref(name)
    }

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Option<&Val>; N] {
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
    fn get(&self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get(name),
            CtxForConstFn::Const(ctx) => ctx.get(name),
        }
    }

    fn remove(&mut self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.remove(name),
            CtxForConstFn::Const(ctx) => ctx.remove(name),
        }
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.put_val(name, val),
            CtxForConstFn::Const(ctx) => ctx.put_val(name, val),
        }
    }

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.put_val_local(name, val),
            CtxForConstFn::Const(ctx) => ctx.put_val_local(name, val),
        }
    }

    fn set_final(&mut self, name: &str) {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_final(name),
            CtxForConstFn::Const(ctx) => ctx.set_final(name),
        }
    }

    fn set_const(&mut self, name: &str) {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_const(name),
            CtxForConstFn::Const(ctx) => ctx.set_const(name),
        }
    }

    fn is_final(&self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_final(name),
            CtxForConstFn::Const(ctx) => ctx.is_final(name),
        }
    }

    fn is_const(&self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_const(name),
            CtxForConstFn::Const(ctx) => ctx.is_const(name),
        }
    }

    fn is_null(&self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_null(name),
            CtxForConstFn::Const(ctx) => ctx.is_null(name),
        }
    }

    fn is_local(&self, name: &str) -> Val {
        match self {
            CtxForConstFn::Free(ctx) => ctx.is_local(name),
            CtxForConstFn::Const(ctx) => ctx.is_local(name),
        }
    }

    fn set_super(&mut self, super_ctx: Option<Symbol>) {
        match self {
            CtxForConstFn::Free(ctx) => ctx.set_super(super_ctx),
            CtxForConstFn::Const(ctx) => ctx.set_super(super_ctx),
        }
    }

    fn get_tagged_ref(&mut self, name: &str) -> Option<TaggedRef<Val>> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_tagged_ref(name),
            CtxForConstFn::Const(ctx) => ctx.get_tagged_ref(name),
        }
    }

    fn get_const_ref(&self, name: &str) -> Option<&Val> {
        match self {
            CtxForConstFn::Free(ctx) => ctx.get_const_ref(name),
            CtxForConstFn::Const(ctx) => ctx.get_const_ref(name),
        }
    }

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Option<&Val>; N] {
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
