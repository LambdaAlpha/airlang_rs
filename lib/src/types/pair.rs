use {
    crate::traits::TryClone,
    std::hash::Hash,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pair<A, B> {
    pub first: A,
    pub second: B,
}

impl<A, B> Pair<A, B> {
    pub(crate) fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A: TryClone, B: TryClone> TryClone for Pair<A, B> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Pair {
            first: self.first.try_clone()?,
            second: self.second.try_clone()?,
        })
    }
}
