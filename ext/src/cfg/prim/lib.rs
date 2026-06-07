use airlang::cfg::CfgMod;
use airlang::cfg::prim::lib::BasePrimLib;
use airlang::semantics::cfg::Cfg;

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
    fn extend(self, cfg: &mut Cfg) {
        self.wasm.extend(cfg);
        self.io.extend(cfg);
        self.file.extend(cfg);
        self.process.extend(cfg);
        self.build.extend(cfg);
        self.base.extend(cfg);
    }
}

pub mod wasm;

pub mod io;

pub mod file;

pub mod process;

pub mod build;
