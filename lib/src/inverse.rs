/// inverse(f) represents the inverse of function f
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Inverse<A> {
    pub func: A,
}

impl<A> Inverse<A> {
    pub fn new(func: A) -> Self {
        Self { func }
    }
}
