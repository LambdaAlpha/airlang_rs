use std::fmt::DebugStruct;

use crate::{
    Ctx,
    Mode,
    Symbol,
    Val,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Composite<Ext> {
    pub(crate) body_mode: Mode,
    pub(crate) body: Val,
    pub(crate) prelude: Ctx,
    pub(crate) input_name: Symbol,
    pub(crate) ext: Ext,
}

impl<T> Composite<T> {
    pub(crate) fn dbg_field(&self, s: &mut DebugStruct) {
        s.field("body_mode", &self.body_mode);
        s.field("body", &self.body);
        s.field("prelude", &self.prelude);
        s.field("input_name", &self.input_name);
    }
}
