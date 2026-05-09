use crate::semantics::cfg::Cfg;

pub trait DynCtx<Key, Value> {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Value>;
    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Value>;
    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Value) -> Option<()>;
}

pub struct Ctx<'a, Value> {
    pub val: &'a mut Value,
    pub const_: bool,
}

impl<'a, Value> Ctx<'a, Value> {
    pub fn new_mut(val: &'a mut Value) -> Self {
        Ctx { val, const_: false }
    }

    pub fn new_const(val: &'a mut Value) -> Self {
        Ctx { val, const_: true }
    }

    pub fn reborrow(&mut self) -> Ctx<'_, Value> {
        Ctx { val: self.val, const_: self.const_ }
    }
}
