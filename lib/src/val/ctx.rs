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
pub struct CtxVal(Box<Ctx>);

impl CtxVal {
    #[allow(unused)]
    pub(crate) fn new(ctx: Box<Ctx>) -> Self {
        Self(ctx)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Ctx> {
        self.0
    }
}

impl From<Ctx> for CtxVal {
    fn from(value: Ctx) -> Self {
        Self(Box::new(value))
    }
}

impl From<CtxVal> for Ctx {
    fn from(value: CtxVal) -> Self {
        *value.0
    }
}

impl Debug for CtxVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ctx::fmt(self, f)
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
