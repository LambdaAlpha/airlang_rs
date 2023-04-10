use crate::traits::TryClone;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Unit;

impl From<&()> for Unit {
    fn from(_: &()) -> Self {
        Unit
    }
}

impl From<()> for Unit {
    fn from(_: ()) -> Self {
        Unit
    }
}

impl Into<()> for &Unit {
    fn into(self) -> () {
        ()
    }
}

impl Into<()> for Unit {
    fn into(self) -> () {
        ()
    }
}

impl TryClone for Unit {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone())
    }
}
