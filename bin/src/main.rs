use std::io::stdout;

use airlang::Air;

use crate::prelude::BinPrelude;
use crate::repl::Repl;
use crate::solver::bin_solver;

fn main() -> std::io::Result<()> {
    Air::init_prelude(Box::new(BinPrelude::default()));
    Air::init_solver(bin_solver());
    let mut repl = Repl::new(stdout());
    repl.run()
}

mod repl;

mod prelude;

mod solver;
