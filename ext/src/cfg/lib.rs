use airlang::cfg::CfgMod;
use airlang::cfg::lib::CoreLib;
use airlang::semantics::cfg::Cfg;

use self::build::BuildLib;
use self::file::FileLib;
use self::io::IoLib;
use self::process::ProcessLib;

#[derive(Default, Clone)]
pub struct StdLib {
    pub io: IoLib,
    pub file: FileLib,
    pub process: ProcessLib,
    pub build: BuildLib,
    pub core: CoreLib,
}

impl CfgMod for StdLib {
    fn extend(self, cfg: &mut Cfg) {
        self.io.extend(cfg);
        self.file.extend(cfg);
        self.process.extend(cfg);
        self.build.extend(cfg);
        self.core.extend(cfg);
    }
}

pub mod io;

pub mod file;

pub mod process;

pub mod build;
