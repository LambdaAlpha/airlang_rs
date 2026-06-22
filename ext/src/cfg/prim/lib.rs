use airlang::cfg::CfgMod;
use airlang::cfg::prim::lib::BasePrimLib;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang::type_::Map;

use self::build::BuildLib;
use self::file::FileLib;
use self::io::IoLib;
use self::process::ProcessLib;
use self::wasm::WasmLib;

#[derive(Default, Copy, Clone)]
pub struct ExtPrimLib {
    pub wasm: WasmLib,
    pub io: IoLib,
    pub file: FileLib,
    pub process: ProcessLib,
    pub build: BuildLib,
    pub base: BasePrimLib,
}

impl CfgMod for ExtPrimLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.wasm.export(cfg);
        self.io.export(cfg);
        self.file.export(cfg);
        self.process.export(cfg);
        self.build.export(cfg);
        self.base.export(cfg);
    }
}

pub mod wasm;

pub mod io;

pub mod file;

pub mod process;

pub mod build;
