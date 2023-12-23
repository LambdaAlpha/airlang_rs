use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::ctx::Ctx;

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

impl Deref for CtxVal {
    type Target = Ctx;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CtxVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
