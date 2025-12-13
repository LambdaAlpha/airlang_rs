use std::ops::BitAnd;

use derive_more::IsVariant;

pub trait DynCtx<Input, Output> {
    fn ref_(&self, input: Input) -> Option<&Output>;
    fn ref_mut(&mut self, input: Input) -> Option<&mut Output>;
    fn set(&mut self, input: Input, value: Output) -> Option<Output>;
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
