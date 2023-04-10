pub(crate) trait TryClone {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized;
}

impl<T: TryClone> TryClone for Box<T> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Box::new((&**self).try_clone()?))
    }
}
