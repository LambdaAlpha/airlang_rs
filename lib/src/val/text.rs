use crate::{
    Text,
    types::wrap::box_wrap,
};

box_wrap!(pub TextVal(Text));

impl From<&TextVal> for Text {
    fn from(value: &TextVal) -> Self {
        Text::clone(value)
    }
}
