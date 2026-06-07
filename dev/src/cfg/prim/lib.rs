use airlang::cfg::CfgMod;
use airlang::semantics::cfg::Cfg;
use airlang_ext::cfg::prim::lib::ExtPrimLib;

use self::lang::LangLib;
use crate::cfg::prim::lib::wasm::WasmLib;

#[derive(Default, Copy, Clone)]
pub struct DevPrimLib {
    pub ext: ExtPrimLib,
    pub lang: LangLib,
    pub wasm: WasmLib,
}

impl CfgMod for DevPrimLib {
    fn extend(self, cfg: &mut Cfg) {
        self.ext.extend(cfg);
        self.lang.extend(cfg);
        self.wasm.extend(cfg);
    }
}

pub mod lang;

pub mod wasm;
