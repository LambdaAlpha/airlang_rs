use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Annotation<A, B> {
    pub note: A,
    pub value: B,
}

impl<A, B> Annotation<A, B> {
    pub fn new(note: A, value: B) -> Self {
        Self { note, value }
    }
}
