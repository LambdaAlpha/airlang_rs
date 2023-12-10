use {
    crate::{
        logic::Prop,
        types::refer::Reader,
    },
    std::{
        fmt::{
            Debug,
            Formatter,
        },
        hash::{
            Hash,
            Hasher,
        },
        ops::Deref,
    },
};

#[derive(Clone, Eq)]
pub struct PropVal(pub(crate) Reader<Prop>);

impl From<Reader<Prop>> for PropVal {
    fn from(value: Reader<Prop>) -> Self {
        PropVal(value)
    }
}

impl PartialEq for PropVal {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }
        *self.0 == *other.0
    }
}

impl Hash for PropVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}

impl Debug for PropVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}
