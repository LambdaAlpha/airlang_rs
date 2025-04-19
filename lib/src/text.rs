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

use crate::syntax::generator::escape_text;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Text(String);

impl Text {
    pub fn push_str(&mut self, s: &str) {
        self.0.push_str(s);
    }

    pub fn push_str_escaped(&mut self, s: &str) {
        escape_text(self, s);
    }
}

impl Deref for Text {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text(String::from(value))
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text(value)
    }
}

impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.0
    }
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        self
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(&self.0, f)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Display>::fmt(&self.0, f)
    }
}
