use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::answer::Answer;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct AnswerVal(Box<Answer>);

impl AnswerVal {
    #[allow(unused)]
    pub(crate) fn new(answer: Box<Answer>) -> Self {
        Self(answer)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Answer> {
        self.0
    }
}

impl From<Answer> for AnswerVal {
    fn from(value: Answer) -> Self {
        Self(Box::new(value))
    }
}

impl From<AnswerVal> for Answer {
    fn from(value: AnswerVal) -> Self {
        *value.0
    }
}

impl Debug for AnswerVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Answer::fmt(self, f)
    }
}

impl Deref for AnswerVal {
    type Target = Answer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AnswerVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
