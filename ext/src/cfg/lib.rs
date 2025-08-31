use airlang::cfg::lib::CoreLib;
use airlang::cfg::lib::Library;
use airlang::semantics::ctx::Ctx;

use self::build::BuildLib;
use self::file::FileLib;
use self::io::IoLib;
use self::process::ProcessLib;

#[derive(Default, Clone)]
pub struct StdLib {
    pub core: CoreLib,
    pub io: IoLib,
    pub file: FileLib,
    pub process: ProcessLib,
    pub build: BuildLib,
}

impl Library for StdLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.core.prelude(ctx);
        self.io.prelude(ctx);
        self.file.prelude(ctx);
        self.process.prelude(ctx);
        self.build.prelude(ctx);
    }
}

pub mod io;

pub mod file;

pub mod process;

pub mod build;
