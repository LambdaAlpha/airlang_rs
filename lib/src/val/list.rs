use crate::{
    list::List,
    syntax::repr::list::ListRepr,
    ReprError,
    Val,
};

pub type ListVal = List<Val>;

impl From<&ListRepr> for ListVal {
    fn from(value: &ListRepr) -> Self {
        value.iter().map(Into::into).collect::<Vec<Val>>().into()
    }
}

impl From<ListRepr> for ListVal {
    fn from(value: ListRepr) -> Self {
        value.into_iter().map(Into::into).collect()
    }
}

impl TryInto<ListRepr> for ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.into_iter()
            .map(TryInto::try_into)
            .collect::<Result<ListRepr, _>>()
    }
}

impl TryInto<ListRepr> for &ListVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ListRepr, Self::Error> {
        self.iter()
            .map(TryInto::try_into)
            .collect::<Result<ListRepr, _>>()
    }
}
