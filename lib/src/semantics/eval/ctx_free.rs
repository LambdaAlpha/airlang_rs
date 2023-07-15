use crate::{
    semantics::{
        eval::ctx::InvariantTag,
        val::{
            RefVal,
            Val,
        },
    },
    types::Keeper,
};

pub(crate) struct CtxFree;

impl CtxFree {
    pub(crate) fn get(ref_val: &RefVal) -> Val {
        let Ok(input) = Keeper::reader(&ref_val.0) else {
            return Val::default();
        };
        input.val.clone()
    }

    pub(crate) fn is_null(ref_val: &RefVal) -> bool {
        Keeper::reader(&ref_val.0).is_err()
    }

    pub(crate) fn is_final(ref_val: &RefVal) -> bool {
        if let Ok(r) = Keeper::reader(&ref_val.0) {
            matches!(&r.tag, InvariantTag::Final | InvariantTag::Const)
        } else {
            false
        }
    }

    pub(crate) fn is_const(ref_val: &RefVal) -> bool {
        if let Ok(r) = Keeper::reader(&ref_val.0) {
            matches!(&r.tag, InvariantTag::Const)
        } else {
            false
        }
    }
}
