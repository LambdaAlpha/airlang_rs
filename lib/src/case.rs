#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Case<F, I, O> {
    pub func: F,
    pub input: I,
    pub output: O,
}

impl<F, I, O> Case<F, I, O> {
    pub fn new(func: F, input: I, output: O) -> Self {
        Self {
            func,
            input,
            output,
        }
    }
}
