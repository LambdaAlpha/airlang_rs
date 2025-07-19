use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Zero;

use crate::type_::bit::Bit;

#[derive(Clone, Default, PartialEq, Eq, Hash, derive_more::Debug, From, Deref, DerefMut)]
#[from(forward)]
#[debug("{_0:?}")]
pub struct Int(BigInt);

impl Int {
    pub(crate) fn new(int: BigInt) -> Self {
        Self(int)
    }

    pub(crate) fn add(self, other: Int) -> Int {
        Int(self.0.add(other.0))
    }

    pub(crate) fn subtract(self, other: Int) -> Int {
        Int(self.0.sub(other.0))
    }

    pub(crate) fn multiply(self, other: Int) -> Int {
        Int(self.0.mul(other.0))
    }

    pub(crate) fn divide(self, other: Int) -> Option<Int> {
        if other.0 == Zero::zero() { None } else { Some(Int(self.0.div(other.0))) }
    }

    pub(crate) fn remainder(self, other: Int) -> Option<Int> {
        if other.0 == Zero::zero() { None } else { Some(Int(self.0.rem(other.0))) }
    }

    pub(crate) fn divide_remainder(self, other: Int) -> Option<(Int, Int)> {
        if other.0 == Zero::zero() {
            None
        } else {
            let (quotient, rem) = self.0.div_rem(&other.0);
            Some((Int(quotient), Int(rem)))
        }
    }

    pub(crate) fn less_than(&self, other: &Int) -> Bit {
        Bit::from(self.0.lt(&other.0))
    }

    pub(crate) fn less_equal(&self, other: &Int) -> Bit {
        Bit::from(self.0.le(&other.0))
    }

    pub(crate) fn greater_than(&self, other: &Int) -> Bit {
        Bit::from(self.0.gt(&other.0))
    }

    pub(crate) fn greater_equal(&self, other: &Int) -> Bit {
        Bit::from(self.0.ge(&other.0))
    }

    pub(crate) fn less_greater(&self, other: &Int) -> Bit {
        Bit::from(self.0.ne(&other.0))
    }
}
