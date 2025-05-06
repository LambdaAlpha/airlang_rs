use std::any::Any;

use crate::Symbol;
use crate::Val;

pub trait Type: Any {
    fn type_name(&self) -> Symbol;
}

pub trait TypeMeta {
    fn arbitrary(&self) -> Val;

    fn arbitrary_type(&self, type1: Symbol) -> Val;
}

pub(crate) mod arbitrary;
