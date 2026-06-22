use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::utils::hint::cold_path;

pub struct Cfg {
    pub map: Map<Key, Val>,
    pub aborted: bool,
}

impl Cfg {
    pub fn new(map: Map<Key, Val>) -> Self {
        Self { map, aborted: false }
    }

    #[cold]
    pub fn abort(&mut self) -> Val {
        self.aborted = true;
        Val::default()
    }

    #[inline(always)]
    pub fn is_aborted(&self) -> bool {
        if self.aborted {
            cold_path();
            true
        } else {
            false
        }
    }
}
