use {
    crate::{
        traits::TryClone,
        types::Bool,
    },
    rug::Integer,
    std::{
        fmt::{
            Debug,
            Display,
            Formatter,
        },
        ops::{
            Add,
            Div,
            Mul,
            Rem,
            Sub,
        },
    },
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Int(Integer);

impl Int {
    pub fn from_sign_string_radix(positive_sign: bool, digits: &str, radix: u8) -> Self {
        let sign = if positive_sign { "+" } else { "-" };
        let s = format!("{sign}{digits}");
        let i = Integer::from_str_radix(&s, radix as i32).unwrap();
        Int(i)
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
        if other.0 == Integer::ZERO {
            None
        } else {
            Some(Int(self.0.div(other.0)))
        }
    }

    pub(crate) fn remainder(self, other: Int) -> Option<Int> {
        if other.0 == Integer::ZERO {
            None
        } else {
            Some(Int(self.0.rem(other.0)))
        }
    }

    pub(crate) fn divide_remainder(self, other: Int) -> Option<(Int, Int)> {
        if other.0 == Integer::ZERO {
            None
        } else {
            let (quotient, rem) = self.0.div_rem(other.0);
            Some((Int(quotient), Int(rem)))
        }
    }

    pub(crate) fn less_than(&self, other: &Int) -> Bool {
        Bool::new(self.0.lt(&other.0))
    }

    pub(crate) fn less_equal(&self, other: &Int) -> Bool {
        Bool::new(self.0.le(&other.0))
    }

    pub(crate) fn greater_than(&self, other: &Int) -> Bool {
        Bool::new(self.0.gt(&other.0))
    }

    pub(crate) fn greater_equal(&self, other: &Int) -> Bool {
        Bool::new(self.0.ge(&other.0))
    }

    pub(crate) fn less_greater(&self, other: &Int) -> Bool {
        Bool::new(self.0.ne(&other.0))
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Display>::fmt(&self.0, f)
    }
}

impl TryClone for Int {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone())
    }
}
