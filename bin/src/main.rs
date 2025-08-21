use std::io::stdout;

use airlang::init_solver;

use crate::repl::Repl;
use crate::solve::bin_solver;

fn main() -> std::io::Result<()> {
    init_solver(bin_solver());
    let mut repl = Repl::new(stdout());
    repl.run()
}

mod repl;

mod prelude;

mod solve;
