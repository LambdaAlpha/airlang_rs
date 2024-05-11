use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

use crate::func::Func;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FuncVal(Rc<Func>);

impl FuncVal {
    pub(crate) fn new(func: Rc<Func>) -> Self {
        Self(func)
    }

    pub(crate) fn unwrap(self) -> Rc<Func> {
        self.0
    }
}

impl From<Func> for FuncVal {
    fn from(value: Func) -> Self {
        FuncVal(Rc::new(value))
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Func::fmt(self, f)
    }
}

impl Deref for FuncVal {
    type Target = Func;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
