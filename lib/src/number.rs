use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use num_bigint::BigInt;

// temporary representation
// int * radix^exp
#[derive(Clone, PartialEq, Hash)]
pub struct Number {
    int: BigInt,
    radix: u8,
    exp: BigInt,
}

impl Number {
    pub(crate) fn new(int: BigInt, radix: u8, exp: BigInt) -> Self {
        Self { int, radix, exp }
    }

    pub(crate) fn int(&self) -> &BigInt {
        &self.int
    }

    pub(crate) fn radix(&self) -> u8 {
        self.radix
    }

    pub(crate) fn exp(&self) -> &BigInt {
        &self.exp
    }
}

impl Eq for Number {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*{}^{}", self.int, self.radix, self.exp)
    }
}
