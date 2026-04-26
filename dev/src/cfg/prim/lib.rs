use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang_ext::cfg::prim::lib::ExtPrimLib;

#[derive(Default, Copy, Clone)]
pub struct DevPrimLib {
    pub ext: ExtPrimLib,
}

impl CfgMod for DevPrimLib {
    fn extend(self, cfg: &mut Cfg) {
        self.ext.extend(cfg);
    }
}
