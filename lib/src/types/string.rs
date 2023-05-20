use {
    smartstring::{
        LazyCompact,
        SmartString,
    },
    std::ops::{
        Deref,
        DerefMut,
    },
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Str(pub(crate) SmartString<LazyCompact>);

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

impl Into<String> for Str {
    fn into(self) -> String {
        self.0.into()
    }
}
