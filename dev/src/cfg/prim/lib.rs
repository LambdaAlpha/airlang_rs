use airlang::cfg::CfgMod;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;
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
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.ext.export(cfg);
        self.lang.export(cfg);
        self.wasm.export(cfg);
    }
}

pub mod lang;

pub mod wasm;
