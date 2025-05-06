use std::io::stdout;

use airlang::AirCell;

use crate::ext::BinExt;
use crate::repl::Repl;

fn main() -> std::io::Result<()> {
    AirCell::set_ext(Box::new(BinExt::default()));
    let mut repl = Repl::new(stdout());
    repl.run()
}

mod ext;

mod repl;

mod prelude;
