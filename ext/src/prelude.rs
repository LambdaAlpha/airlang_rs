pub use self::build::BuildPrelude;
pub use self::file::FilePrelude;
pub use self::io::IoPrelude;
pub use self::process::ProcessPrelude;

airlang::_____!();

use airlang::prelude::CorePrelude;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;

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

mod io;

mod file;

mod process;

mod build;
