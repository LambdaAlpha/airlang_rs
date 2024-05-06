#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Either<L, R> {
    Left(L),
    Right(R),
}

#[allow(unused)]
impl<A, B> Either<A, B> {
    pub(crate) fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    pub(crate) fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
    pub(crate) fn unwrap_left(self) -> A {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => {
                panic!("called `Either::unwrap_left()` on a `Either::Right(_)` value")
            }
        }
    }
    pub(crate) fn unwrap_right(self) -> B {
        match self {
            Either::Right(r) => r,
            Either::Left(_) => {
                panic!("called `Either::unwrap_right()` on a `Either::Left(_)` value")
            }
        }
    }
}
