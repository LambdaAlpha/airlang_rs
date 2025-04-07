/// equiv(f) represents a function which transform a value `x` to its equivalent `y`,
/// in the sense that `f ; x` and `f ; y` always produce the same output
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Equiv<A> {
    pub func: A,
}

impl<A> Equiv<A> {
    pub fn new(func: A) -> Self {
        Self { func }
    }
}
