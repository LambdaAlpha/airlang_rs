use airlang::cfg::CfgMod;
use airlang::cfg::lib::CoreLib;
use airlang::cfg::lib::Library;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::memo::Memo;

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
    fn extend(self, cfg: &Cfg) {
        self.io.extend(cfg);
        self.file.extend(cfg);
        self.process.extend(cfg);
        self.build.extend(cfg);
        self.core.extend(cfg);
    }
}

impl Library for StdLib {
    fn prelude(&self, memo: &mut Memo) {
        self.io.prelude(memo);
        self.file.prelude(memo);
        self.process.prelude(memo);
        self.build.prelude(memo);
        self.core.prelude(memo);
    }
}

pub mod io;

pub mod file;

pub mod process;

pub mod build;
