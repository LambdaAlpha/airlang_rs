use std::ops::Neg;

use bigdecimal::BigDecimal;
use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;

// todo design
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug, From, Deref, DerefMut)]
#[from(forward)]
#[debug("{_0:E}")]
pub struct Decimal(BigDecimal);

impl Decimal {
    pub(crate) fn new(d: BigDecimal) -> Self {
        Self(d)
    }

    #[expect(dead_code)]
    pub(crate) fn unwrap(self) -> BigDecimal {
        self.0
    }
}

impl Neg for Decimal {
    type Output = Decimal;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
