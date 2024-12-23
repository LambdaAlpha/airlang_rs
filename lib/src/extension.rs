use std::{
    any::Any,
    fmt::Debug,
    hash::Hash,
};

use crate::traits::dyn_safe::dyn_any_clone_eq_hash;

dyn_any_clone_eq_hash!(pub ValExt : Any);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct UnitExt;
