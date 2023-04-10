use {
    crate::traits::TryClone,
    std::hash::Hash,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Call<A, B> {
    pub func: A,
    pub input: B,
}

impl<A, B> Call<A, B> {
    pub(crate) fn new(func: A, input: B) -> Self {
        Self { func, input }
    }
}

impl<A: TryClone, B: TryClone> TryClone for Call<A, B> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Call {
            func: self.func.try_clone()?,
            input: self.input.try_clone()?,
        })
    }
}
