#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Change<F, T> {
    pub from: F,
    pub to: T,
}

impl<F, T> Change<F, T> {
    pub fn new(from: F, to: T) -> Change<F, T> {
        Change { from, to }
    }
}
