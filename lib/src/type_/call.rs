#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Call<A, B> {
    pub reverse: bool,
    pub func: A,
    pub input: B,
}

impl<A, B> Call<A, B> {
    pub fn new(reverse: bool, func: A, input: B) -> Self {
        Self { reverse, func, input }
    }
}
