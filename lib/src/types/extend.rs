use crate::traits::TryClone;

#[derive(Debug, PartialEq, Eq)]
pub struct Extend;

impl TryClone for Extend {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Extend)
    }
}
