use crate::{
    semantics::{
        ctx::{
            CtxTrait,
            InvariantTag,
            TaggedRef,
            TaggedVal,
        },
        ctx_access::{
            constant::CtxForConstFn,
            mutable::CtxForMutableFn,
            CtxAccessor,
        },
        val::{
            RefVal,
            Val,
        },
    },
    types::{
        Either,
        Keeper,
        Owner,
        Symbol,
    },
};

pub(crate) struct FreeCtx;

impl FreeCtx {
    pub(crate) fn get_val_ref(ref_val: &RefVal) -> Val {
        let Ok(input) = Keeper::reader(&ref_val.0) else {
            return Val::default();
        };
        input.val.clone()
    }

    pub(crate) fn is_null_ref(ref_val: &RefVal) -> bool {
        Keeper::reader(&ref_val.0).is_err()
    }

    pub(crate) fn is_final_ref(ref_val: &RefVal) -> bool {
        if let Ok(r) = Keeper::reader(&ref_val.0) {
            matches!(&r.tag, InvariantTag::Final | InvariantTag::Const)
        } else {
            false
        }
    }

    pub(crate) fn is_const_ref(ref_val: &RefVal) -> bool {
        if let Ok(r) = Keeper::reader(&ref_val.0) {
            matches!(&r.tag, InvariantTag::Const)
        } else {
            false
        }
    }

    pub(crate) fn set_final_ref(ref_val: &RefVal) {
        let Ok(mut o) = Keeper::owner(&ref_val.0) else {
            return;
        };
        if !matches!(o.tag, InvariantTag::None) {
            return;
        }
        o.tag = InvariantTag::Final;
    }

    pub(crate) fn set_const_ref(ref_val: &RefVal) {
        let Ok(mut o) = Keeper::owner(&ref_val.0) else {
            return;
        };
        o.tag = InvariantTag::Const;
    }

    pub(crate) fn remove_ref(ref_val: &RefVal) -> Val {
        let Ok(o) = Keeper::owner(&ref_val.0) else {
            return Val::default();
        };
        if !matches!(o.tag, InvariantTag::None) {
            return Val::default();
        }
        Owner::move_data(o).val
    }
}

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

    fn set_super(&mut self, _super_ctx: Option<Either<Symbol, RefVal>>) {}

    fn get_ref<T, F>(&mut self, _name: &str, f: F) -> T
    where
        F: FnOnce(Option<TaggedRef<Val>>) -> T,
    {
        f(None)
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
        CtxForConstFn::Free
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Free
    }
}
