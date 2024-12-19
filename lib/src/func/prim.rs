use std::{
    fmt::Debug,
    hash::Hash,
};

use crate::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Primitive {
    pub(crate) is_extension: bool,
    pub(crate) id: Symbol,
}
