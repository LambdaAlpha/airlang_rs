use std::{
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    hash::Hash,
    str::FromStr,
};

use num_bigint::{
    BigInt,
    BigUint,
    Sign,
};
use num_traits::Zero;

// temporary representation
// int * 10^exp
#[derive(Clone, PartialEq, Hash)]
pub struct Float(Box<FloatInner>);

#[derive(Clone, PartialEq, Hash)]
struct FloatInner {
    int: BigInt,
    exp: BigInt,
}

impl Float {
    pub(crate) fn from_parts(
        sign: bool,
        integral: &str,
        fractional: &str,
        exp_sign: bool,
        exp: &str,
    ) -> Self {
        let sign = if sign { Sign::Plus } else { Sign::Minus };
        let mut int = String::from(integral);
        int.push_str(fractional);
        let int = BigUint::from_str(&int).unwrap();
        let int = BigInt::from_biguint(sign, int);
        let exp_sign = if exp_sign { Sign::Plus } else { Sign::Minus };
        let exp = if exp.is_empty() {
            Zero::zero()
        } else {
            BigUint::from_str(exp).unwrap()
        };
        let exp = BigInt::from_biguint(exp_sign, exp);
        let exp = exp - fractional.len();
        Self(Box::new(FloatInner { int, exp }))
    }
}

impl Eq for Float {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Display for FloatInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}e{}", self.int, self.exp)
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for FloatInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Display>::fmt(self, f)
    }
}

impl Debug for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Display>::fmt(self, f)
    }
}
