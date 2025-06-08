use crate::type_::Text;
use crate::type_::wrap::box_wrap;

box_wrap!(pub TextVal(Text));

impl From<&TextVal> for Text {
    fn from(value: &TextVal) -> Self {
        Text::clone(value)
    }
}
