use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Annotated<A, B> {
    pub annotation: A,
    pub value: B,
}

impl<A, B> Annotated<A, B> {
    pub fn new(annotation: A, value: B) -> Self {
        Self { annotation, value }
    }
}
