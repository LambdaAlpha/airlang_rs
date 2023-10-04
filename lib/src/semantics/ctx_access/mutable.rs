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
        Val,
    },
    types::{
        Bool,
        Symbol,
    },
};

pub(crate) struct MutableCtx<'a>(pub(crate) &'a mut Ctx);

pub(crate) enum CtxForMutableFn<'a> {
    Free(FreeCtx),
    Const(ConstCtx<'a>),
    Mutable(MutableCtx<'a>),
}

impl<'a> CtxTrait for MutableCtx<'a> {
    fn get(&self, name: &str) -> Val {
        self.0.get(name)
    }

    fn remove(&mut self, name: &str) -> Val {
        self.0.remove(name)
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        self.0.put_val(name, val)
    }

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val {
        self.0.put_val_local(name, val)
    }

    fn set_final(&mut self, name: &str) {
        self.0.set_final(name);
    }

    fn set_const(&mut self, name: &str) {
        self.0.set_const(name);
    }

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

    fn set_super(&mut self, super_ctx: Option<Symbol>) {
        self.0.super_ctx = super_ctx;
    }

    fn get_super(&self) -> Option<Symbol> {
        self.0.super_ctx.clone()
    }

    fn get_tagged_ref(&mut self, name: &str) -> Option<TaggedRef<Val>> {
        self.0.get_tagged_ref(false, name)
    }

    fn get_const_ref(&self, name: &str) -> Option<&Val> {
        self.0.get_const_ref(name)
    }

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Option<&Val>; N] {
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
        CtxForConstFn::Const(ConstCtx(self.0))
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Mutable(self.reborrow())
    }
}

impl<'a> CtxTrait for CtxForMutableFn<'a> {
    fn get(&self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get(name),
            CtxForMutableFn::Const(ctx) => ctx.get(name),
            CtxForMutableFn::Mutable(ctx) => ctx.get(name),
        }
    }

    fn remove(&mut self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.remove(name),
            CtxForMutableFn::Const(ctx) => ctx.remove(name),
            CtxForMutableFn::Mutable(ctx) => ctx.remove(name),
        }
    }

    fn put_val(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.put_val(name, val),
            CtxForMutableFn::Const(ctx) => ctx.put_val(name, val),
            CtxForMutableFn::Mutable(ctx) => ctx.put_val(name, val),
        }
    }

    fn put_val_local(&mut self, name: Symbol, val: TaggedVal) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.put_val_local(name, val),
            CtxForMutableFn::Const(ctx) => ctx.put_val_local(name, val),
            CtxForMutableFn::Mutable(ctx) => ctx.put_val_local(name, val),
        }
    }

    fn set_final(&mut self, name: &str) {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_final(name),
            CtxForMutableFn::Const(ctx) => ctx.set_final(name),
            CtxForMutableFn::Mutable(ctx) => ctx.set_final(name),
        }
    }

    fn set_const(&mut self, name: &str) {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_const(name),
            CtxForMutableFn::Const(ctx) => ctx.set_const(name),
            CtxForMutableFn::Mutable(ctx) => ctx.set_const(name),
        }
    }

    fn is_final(&self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_final(name),
            CtxForMutableFn::Const(ctx) => ctx.is_final(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_final(name),
        }
    }

    fn is_const(&self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_const(name),
            CtxForMutableFn::Const(ctx) => ctx.is_const(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_const(name),
        }
    }

    fn is_null(&self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_null(name),
            CtxForMutableFn::Const(ctx) => ctx.is_null(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_null(name),
        }
    }

    fn is_local(&self, name: &str) -> Val {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.is_local(name),
            CtxForMutableFn::Const(ctx) => ctx.is_local(name),
            CtxForMutableFn::Mutable(ctx) => ctx.is_local(name),
        }
    }

    fn set_super(&mut self, super_ctx: Option<Symbol>) {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.set_super(super_ctx),
            CtxForMutableFn::Const(ctx) => ctx.set_super(super_ctx),
            CtxForMutableFn::Mutable(ctx) => ctx.set_super(super_ctx),
        }
    }

    fn get_super(&self) -> Option<Symbol> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_super(),
            CtxForMutableFn::Const(ctx) => ctx.get_super(),
            CtxForMutableFn::Mutable(ctx) => ctx.get_super(),
        }
    }

    fn get_tagged_ref(&mut self, name: &str) -> Option<TaggedRef<Val>> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_tagged_ref(name),
            CtxForMutableFn::Const(ctx) => ctx.get_tagged_ref(name),
            CtxForMutableFn::Mutable(ctx) => ctx.get_tagged_ref(name),
        }
    }

    fn get_const_ref(&self, name: &str) -> Option<&Val> {
        match self {
            CtxForMutableFn::Free(ctx) => ctx.get_const_ref(name),
            CtxForMutableFn::Const(ctx) => ctx.get_const_ref(name),
            CtxForMutableFn::Mutable(ctx) => ctx.get_const_ref(name),
        }
    }

    fn get_many_const_ref<const N: usize>(&self, names: [&str; N]) -> [Option<&Val>; N] {
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
            CtxForMutableFn::Mutable(ctx) => CtxForConstFn::Const(ConstCtx(ctx.0)),
        }
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        self.reborrow()
    }
}

impl<'a> MutableCtx<'a> {
    pub(crate) fn reborrow(&mut self) -> MutableCtx {
        MutableCtx(self.0)
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
