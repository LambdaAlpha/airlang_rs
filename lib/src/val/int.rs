use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Int,
    box_wrap,
};

box_wrap!(pub IntVal(Int));

impl Debug for IntVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Int::fmt(self, f)
    }
}
