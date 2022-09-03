mod repl;

use env_logger;

fn main() {
    env_logger::init();

    repl::repl();
}
