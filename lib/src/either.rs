#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Either<This, That> {
    This(This),
    That(That),
}

impl<This, That> Either<This, That> {
    pub fn is_this(&self) -> bool {
        matches!(self, Either::This(_))
    }

    pub fn is_that(&self) -> bool {
        matches!(self, Either::That(_))
    }

    pub fn unwrap_this(self) -> This {
        match self {
            Either::This(l) => l,
            Either::That(_) => {
                panic!("called `Either::unwrap_this()` on a `Either::That(_)` value")
            }
        }
    }

    pub fn unwrap_that(self) -> That {
        match self {
            Either::That(r) => r,
            Either::This(_) => {
                panic!("called `Either::unwrap_that()` on a `Either::This(_)` value")
            }
        }
    }
}
