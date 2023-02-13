use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Letter(String);

impl Letter {
    pub(crate) fn new(s: String) -> Self {
        Letter(s)
    }
}

impl Deref for Letter {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
