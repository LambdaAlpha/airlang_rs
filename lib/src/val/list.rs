use crate::{
    list::List,
    syntax::repr::list::ListRepr,
    ReprError,
    Val,
};

pub type ListVal = List<Val>;

impl From<&ListRepr> for ListVal {
    fn from(value: &ListRepr) -> Self {
        value.iter().map(|v| v.into()).collect::<Vec<Val>>().into()
    }
}

impl From<ListRepr> for ListVal {
    fn from(value: ListRepr) -> Self {
        value.into_iter().map(|v| v.into()).collect()
    }
}

impl TryInto<ListRepr> for ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.into_iter().map(|v| v.try_into()).try_collect()
    }
}

impl TryInto<ListRepr> for &ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.iter().map(|v| v.try_into()).try_collect()
    }
}
