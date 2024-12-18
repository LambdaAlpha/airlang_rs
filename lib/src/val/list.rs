use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Val,
    box_wrap,
    list::List,
    syntax::{
        ReprError,
        repr::list::ListRepr,
    },
};

box_wrap!(pub ListVal(List<Val>));

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
