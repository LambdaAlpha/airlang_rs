use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::Deref,
};

use crate::problem::Answer;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct AnswerVal(pub(crate) Box<Answer>);

impl From<Answer> for AnswerVal {
    fn from(value: Answer) -> Self {
        Self(Box::new(value))
    }
}

impl From<Box<Answer>> for AnswerVal {
    fn from(value: Box<Answer>) -> Self {
        Self(value)
    }
}

impl Debug for AnswerVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}

impl Deref for AnswerVal {
    type Target = Answer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
