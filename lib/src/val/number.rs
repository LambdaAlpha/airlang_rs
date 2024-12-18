use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Number,
    box_wrap,
};

box_wrap!(pub NumberVal(Number));

impl Debug for NumberVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Number::fmt(self, f)
    }
}
