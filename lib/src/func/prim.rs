use std::{
    fmt::{
        Debug,
        DebugStruct,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
};

use crate::Symbol;

#[derive(Clone)]
pub(crate) struct Primitive<Ext> {
    pub(crate) is_extension: bool,
    pub(crate) id: Symbol,
    pub(crate) ext: Ext,
}

impl<F> PartialEq for Primitive<F> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.is_extension == other.is_extension
    }
}

impl<F> Eq for Primitive<F> {}

impl<F> Hash for Primitive<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.is_extension.hash(state);
    }
}

impl<T> Debug for Primitive<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Primitive");
        self.dbg_field(&mut s);
        s.finish()
    }
}

impl<T> Primitive<T> {
    pub(crate) fn dbg_field(&self, s: &mut DebugStruct) {
        s.field("id", &self.id);
        s.field("is_extension", &self.is_extension);
    }
}
