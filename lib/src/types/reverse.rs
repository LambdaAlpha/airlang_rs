use {
    crate::traits::TryClone,
    std::hash::Hash,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Reverse<A, B> {
    pub func: A,
    pub output: B,
}

impl<A, B> Reverse<A, B> {
    pub(crate) fn new(func: A, output: B) -> Self {
        Self { func, output }
    }
}

impl<A: TryClone, B: TryClone> TryClone for Reverse<A, B> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Reverse {
            func: self.func.try_clone()?,
            output: self.output.try_clone()?,
        })
    }
}
