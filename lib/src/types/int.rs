use {
    crate::traits::TryClone,
    rug::Integer,
    std::fmt::{
        Debug,
        Display,
        Formatter,
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
