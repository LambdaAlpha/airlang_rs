use std::{
    ops::Deref,
    string::String,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Str(String);

impl From<String> for Str {
    fn from(value: String) -> Self {
        Str(value)
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Str(value.to_owned())
    }
}

impl Deref for Str {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
