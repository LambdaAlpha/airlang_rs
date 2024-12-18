use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Byte,
    box_wrap,
};

box_wrap!(pub ByteVal(Byte));

impl Debug for ByteVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Byte::fmt(self, f)
    }
}
