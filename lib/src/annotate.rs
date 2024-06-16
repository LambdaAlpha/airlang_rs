use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Annotate<A, B> {
    pub note: A,
    pub value: B,
}

impl<A, B> Annotate<A, B> {
    pub fn new(note: A, value: B) -> Self {
        Self { note, value }
    }
}
