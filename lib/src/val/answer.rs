use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    answer::Answer,
    box_wrap,
};

box_wrap!(pub AnswerVal(Answer));

impl Debug for AnswerVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Answer::fmt(self, f)
    }
}
