use crate::{
    semantics::{
        ctx::{
            CtxTrait,
            TaggedRef,
            TaggedVal,
        },
        ctx_access::{
            constant::CtxForConstFn,
            mutable::CtxForMutableFn,
            CtxAccessor,
        },
        val::Val,
    },
    types::Symbol,
};

pub(crate) struct FreeCtx;

impl CtxTrait for FreeCtx {
    fn get(&mut self, _name: &str) -> Val {
        Val::default()
    }

    fn is_null(&mut self, _name: &str) -> Val {
        Val::default()
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

    fn is_final(&mut self, _name: &str) -> Val {
        Val::default()
    }

    fn is_const(&mut self, _name: &str) -> Val {
        Val::default()
    }

    fn set_super(&mut self, _super_ctx: Option<Symbol>) {}

    fn get_tagged_ref<T, F>(&mut self, _name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>) -> T,
    {
        f(None)
    }

    fn get_const_ref(&self, _name: &str) -> Option<&Val> {
        None
    }

    fn get_many_const_ref<const N: usize>(&self, _names: [&str; N]) -> [Option<&Val>; N] {
        [None; N]
    }
}

impl CtxAccessor for FreeCtx {
    fn is_ctx_free(&self) -> bool {
        true
    }

    fn is_ctx_const(&self) -> bool {
        true
    }

    fn for_const_fn(&mut self) -> CtxForConstFn {
        CtxForConstFn::Free(FreeCtx)
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Free(FreeCtx)
    }
}
