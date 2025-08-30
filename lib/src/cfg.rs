use crate::semantics::cfg::Cfg;

pub trait CfgModule {
    fn extend(self, cfg: &Cfg);
}

impl<T: CfgModule> From<T> for Cfg {
    fn from(t: T) -> Cfg {
        let cfg = Cfg::default();
        t.extend(&cfg);
        cfg
    }
}

#[derive(Default, Clone)]
pub struct CoreCfg {}

impl CfgModule for CoreCfg {
    fn extend(self, _cfg: &Cfg) {}
}
