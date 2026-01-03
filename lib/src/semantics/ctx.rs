use std::ops::BitAnd;

use derive_more::IsVariant;

pub trait DynCtx<Key, Value> {
    fn ref_(&self, key: Key) -> Option<&Value>;
    fn ref_mut(&mut self, key: Key) -> Option<&mut Value>;
    fn set(&mut self, key: Key, value: Value) -> Option<Value>;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, IsVariant)]
pub enum CtxAccess {
    Free,
    Const,
    #[default]
    Mut,
}

impl BitAnd for CtxAccess {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        if self == CtxAccess::Mut || rhs == CtxAccess::Mut {
            return CtxAccess::Mut;
        }
        if self == CtxAccess::Const || rhs == CtxAccess::Const {
            return CtxAccess::Const;
        }
        CtxAccess::Free
    }
}
