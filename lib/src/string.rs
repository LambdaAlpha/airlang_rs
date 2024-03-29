use std::{
    borrow::Borrow,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Str(String);

impl Str {
    pub fn push_str(&mut self, s: &str) {
        self.0.push_str(s);
    }
}

impl Deref for Str {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Str {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Str(String::from(value))
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        Str(value)
    }
}

impl From<Str> for String {
    fn from(value: Str) -> Self {
        value.0
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        self
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(&self.0, f)
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Display>::fmt(&self.0, f)
    }
}
