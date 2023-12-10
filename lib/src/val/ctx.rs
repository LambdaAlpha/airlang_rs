use {
    crate::ctx::Ctx,
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        ops::Deref,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtxVal(pub(crate) Box<Ctx>);

impl From<Box<Ctx>> for CtxVal {
    fn from(value: Box<Ctx>) -> Self {
        CtxVal(value)
    }
}

impl Debug for CtxVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}
