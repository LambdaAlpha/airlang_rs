use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::Text;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TextVal(Box<Text>);

impl TextVal {
    #[allow(unused)]
    pub(crate) fn new(str: Box<Text>) -> Self {
        Self(str)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Text> {
        self.0
    }
}

impl From<Text> for TextVal {
    fn from(value: Text) -> Self {
        TextVal(Box::new(value))
    }
}

impl From<TextVal> for Text {
    fn from(value: TextVal) -> Self {
        *value.0
    }
}

impl From<&TextVal> for Text {
    fn from(value: &TextVal) -> Self {
        Text::clone(value)
    }
}

impl Debug for TextVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Text::fmt(self, f)
    }
}

impl Deref for TextVal {
    type Target = Text;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
