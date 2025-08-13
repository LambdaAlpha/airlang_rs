use std::io::stdout;

use airlang::init_prelude;
use airlang::init_solver;

use crate::prelude::BinPrelude;
use crate::repl::Repl;
use crate::solve::bin_solver;

fn main() -> std::io::Result<()> {
    init_prelude(Box::new(BinPrelude::default()));
    init_solver(bin_solver());
    let mut repl = Repl::new(stdout());
    repl.run()
}

mod repl;

mod prelude;

mod solve;
