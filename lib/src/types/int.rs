use {
    crate::types::Bool,
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

#[allow(unused)]
impl Int {
    pub fn from_sign_string_radix(positive_sign: bool, digits: &str, radix: u8) -> Self {
        let sign = if positive_sign { "+" } else { "-" };
        let s = format!("{sign}{digits}");
        let i = Integer::from_str_radix(&s, radix as i32).unwrap();
        Int(i)
    }

    pub(crate) fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    pub(crate) fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    pub(crate) fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    pub(crate) fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    pub(crate) fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    pub(crate) fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    pub(crate) fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    pub(crate) fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    pub(crate) fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    pub(crate) fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    pub(crate) fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    pub(crate) fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
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

impl From<i8> for Int {
    fn from(value: i8) -> Self {
        Int(value.into())
    }
}

impl From<i16> for Int {
    fn from(value: i16) -> Self {
        Int(value.into())
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        Int(value.into())
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Int(value.into())
    }
}

impl From<i128> for Int {
    fn from(value: i128) -> Self {
        Int(value.into())
    }
}

impl From<isize> for Int {
    fn from(value: isize) -> Self {
        Int(value.into())
    }
}

impl From<u8> for Int {
    fn from(value: u8) -> Self {
        Int(value.into())
    }
}

impl From<u16> for Int {
    fn from(value: u16) -> Self {
        Int(value.into())
    }
}

impl From<u32> for Int {
    fn from(value: u32) -> Self {
        Int(value.into())
    }
}

impl From<u64> for Int {
    fn from(value: u64) -> Self {
        Int(value.into())
    }
}

impl From<u128> for Int {
    fn from(value: u128) -> Self {
        Int(value.into())
    }
}

impl From<usize> for Int {
    fn from(value: usize) -> Self {
        Int(value.into())
    }
}
