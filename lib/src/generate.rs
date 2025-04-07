/// generate(f) represents the image of the function f
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Generate<F> {
    pub func: F,
}

impl<F> Generate<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}
