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

use crate::{
    list::List,
    syntax::repr::list::ListRepr,
    ReprError,
    Val,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ListVal(Box<List<Val>>);

impl ListVal {
    #[allow(unused)]
    pub(crate) fn new(list: Box<List<Val>>) -> Self {
        Self(list)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<List<Val>> {
        self.0
    }
}

impl From<List<Val>> for ListVal {
    fn from(value: List<Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<ListVal> for List<Val> {
    fn from(value: ListVal) -> Self {
        *value.0
    }
}

impl From<&ListRepr> for ListVal {
    fn from(value: &ListRepr) -> Self {
        let list = value.iter().map(Into::into).collect();
        Self(Box::new(list))
    }
}

impl From<ListRepr> for ListVal {
    fn from(value: ListRepr) -> Self {
        let list = value.into_iter().map(Into::into).collect();
        Self(Box::new(list))
    }
}

impl TryInto<ListRepr> for &ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.0.iter().map(TryInto::try_into).collect()
    }
}

impl TryInto<ListRepr> for ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.0.into_iter().map(TryInto::try_into).collect()
    }
}

impl Debug for ListVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        List::fmt(self, f)
    }
}

impl Deref for ListVal {
    type Target = List<Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ListVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
