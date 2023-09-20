use {
    smartstring::{
        LazyCompact,
        SmartString,
    },
    std::{
        borrow::Borrow,
        ops::{
            Deref,
            DerefMut,
        },
    },
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Str(SmartString<LazyCompact>);

impl Deref for Str {
    type Target = str;
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
        Str(SmartString::from(value))
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        Str(SmartString::from(value))
    }
}

impl ToString for Str {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Str> for String {
    fn from(value: Str) -> Self {
        value.0.into()
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        self
    }
}
