/// abstract(f) represents the abstraction of function f,
/// which transforms a value abstraction to a value abstraction
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Abstract<F> {
    pub func: F,
}

impl<F> Abstract<F> {
    pub fn new(func: F) -> Self {
        Self { func }
    }
}
