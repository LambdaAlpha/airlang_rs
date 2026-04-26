use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang_ext::cfg::prim::lib::ExtPrimLib;

use self::lang::LangLib;

#[derive(Default, Copy, Clone)]
pub struct DevPrimLib {
    pub ext: ExtPrimLib,
    pub lang: LangLib,
}

impl CfgMod for DevPrimLib {
    fn extend(self, cfg: &mut Cfg) {
        self.ext.extend(cfg);
        self.lang.extend(cfg);
    }
}

pub mod lang;
