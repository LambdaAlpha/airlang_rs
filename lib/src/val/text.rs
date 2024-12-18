use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Text,
    box_wrap,
};

box_wrap!(pub TextVal(Text));

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
