use crate::{
    semantics::val::Val,
    types::{
        Int,
        Str,
    },
};

pub(crate) fn version_code() -> Val {
    Val::Int(Int::from_sign_string_radix(true, "3", 10))
}

pub(crate) fn version_name() -> Val {
    Val::String(Str::from("0.0.3"))
}
