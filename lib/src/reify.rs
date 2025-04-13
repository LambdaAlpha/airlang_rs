/// return a computable equivalent function of func
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Reify<F> {
    pub func: F,
}

impl<F> Reify<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}
