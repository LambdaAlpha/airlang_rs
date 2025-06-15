use airlang::prelude::CorePrelude;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;

use self::build::BuildPrelude;
use self::file::FilePrelude;
use self::io::IoPrelude;
use self::process::ProcessPrelude;

#[derive(Default)]
pub struct StdPrelude {
    pub core: CorePrelude,
    pub io: IoPrelude,
    pub file: FilePrelude,
    pub process: ProcessPrelude,
    pub build: BuildPrelude,
}

impl Prelude for StdPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.core.put(ctx);
        self.io.put(ctx);
        self.file.put(ctx);
        self.process.put(ctx);
        self.build.put(ctx);
    }
}

pub mod io;

pub mod file;

pub mod process;

pub mod build;
