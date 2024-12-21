use crate::{
    Val,
    list::List,
    syntax::{
        ReprError,
        repr::list::ListRepr,
    },
    types::wrap::box_wrap,
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
