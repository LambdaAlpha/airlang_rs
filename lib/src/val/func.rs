use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    ops::Deref,
    rc::Rc,
};

use crate::func::Func;

#[derive(Clone, Eq)]
pub struct FuncVal(pub(crate) Rc<Func>);

impl From<Rc<Func>> for FuncVal {
    fn from(value: Rc<Func>) -> Self {
        FuncVal(value)
    }
}

impl From<Func> for FuncVal {
    fn from(value: Func) -> Self {
        FuncVal(Rc::new(value))
    }
}

impl PartialEq for FuncVal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for FuncVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}

impl Deref for FuncVal {
    type Target = Func;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
