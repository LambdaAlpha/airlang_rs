use {
    smartstring::alias::CompactString,
    std::ops::Deref,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Letter(CompactString);

impl Letter {
    pub(crate) fn from_str(s: &str) -> Self {
        Letter(CompactString::from(s))
    }
}

impl Deref for Letter {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for Letter {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Into<String> for Letter {
    fn into(self) -> String {
        self.0.into()
    }
}
