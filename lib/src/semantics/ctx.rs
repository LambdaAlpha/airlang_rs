use crate::semantics::cfg::Cfg;

pub trait DynCtx<Key, Value> {
    fn ref_(&self, cfg: &mut Cfg, key: Key) -> Option<&Value>;
    fn ref_mut(&mut self, cfg: &mut Cfg, key: Key) -> Option<&mut Value>;
    fn set(&mut self, cfg: &mut Cfg, key: Key, value: Value) -> Option<()>;
}
