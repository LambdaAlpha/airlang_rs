use std::io::stdout;

use crate::repl::Repl;

fn main() -> std::io::Result<()> {
    let mut repl = Repl::new(stdout());
    repl.run()
}

mod repl;

mod prelude;
