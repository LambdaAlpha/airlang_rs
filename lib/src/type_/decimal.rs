use std::num::NonZeroU64;
use std::ops::Neg;

use bigdecimal::BigDecimal;
use bigdecimal::Context;
use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;
use num_bigint::Sign;
use num_traits::One;
use num_traits::Zero;

use crate::type_::Bit;

// todo design
#[derive(Clone, PartialEq, Eq, Hash, From, Deref, DerefMut)]
#[from(forward)]
pub struct Decimal(BigDecimal);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DecimalConfig {
    rounding_mode: RoundingMode,
    precision: NonZeroU64,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RoundingMode {
    Infinity,
    Zero,
    Positive,
    Negative,
    HalfInfinity,
    HalfZero,
    HalfEven,
}

impl Decimal {
    pub(crate) fn new(d: BigDecimal) -> Self {
        Self(d)
    }

    #[expect(dead_code)]
    pub(crate) fn unwrap(self) -> BigDecimal {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    pub fn sign(&self) -> Sign {
        self.0.sign()
    }

    pub fn add(self, other: Decimal, cfg: DecimalConfig) -> Decimal {
        let output = self.0 + other.0;
        let output = cfg.into_inner().round_decimal(output);
        Decimal(output)
    }

    pub fn subtract(self, other: Decimal, cfg: DecimalConfig) -> Decimal {
        let output = self.0 - other.0;
        let output = cfg.into_inner().round_decimal(output);
        Decimal(output)
    }

    pub fn multiply(self, other: Decimal, cfg: DecimalConfig) -> Decimal {
        let output = self.0 * other.0;
        let output = cfg.into_inner().round_decimal(output);
        Decimal(output)
    }

    pub fn divide(self, other: Decimal, cfg: DecimalConfig) -> Decimal {
        let output = self.0 / other.0;
        let output = cfg.into_inner().round_decimal(output);
        Decimal(output)
    }

    pub fn less_than(&self, other: &Decimal) -> Bit {
        Bit::from(self.0 < other.0)
    }

    pub fn less_equal(&self, other: &Decimal) -> Bit {
        Bit::from(self.0 <= other.0)
    }

    pub fn greater_than(&self, other: &Decimal) -> Bit {
        Bit::from(self.0 > other.0)
    }

    pub fn greater_equal(&self, other: &Decimal) -> Bit {
        Bit::from(self.0 >= other.0)
    }

    pub fn less_greater(&self, other: &Decimal) -> Bit {
        Bit::from(self.0 != other.0)
    }
}

impl DecimalConfig {
    pub fn new(precision: NonZeroU64, rounding_mode: RoundingMode) -> Self {
        Self { precision, rounding_mode }
    }

    pub(crate) fn into_inner(self) -> Context {
        Context::new(self.precision, self.rounding_mode.into_inner())
    }
}

impl RoundingMode {
    pub(crate) fn into_inner(self) -> bigdecimal::RoundingMode {
        match self {
            RoundingMode::Infinity => bigdecimal::RoundingMode::Up,
            RoundingMode::Zero => bigdecimal::RoundingMode::Down,
            RoundingMode::Positive => bigdecimal::RoundingMode::Ceiling,
            RoundingMode::Negative => bigdecimal::RoundingMode::Floor,
            RoundingMode::HalfInfinity => bigdecimal::RoundingMode::HalfUp,
            RoundingMode::HalfZero => bigdecimal::RoundingMode::HalfDown,
            RoundingMode::HalfEven => bigdecimal::RoundingMode::HalfEven,
        }
    }
}

impl Neg for Decimal {
    type Output = Decimal;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
