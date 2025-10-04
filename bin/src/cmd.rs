use std::env::args;
use std::io::Write;
use std::io::stderr;
use std::io::stdout;

use airlang::Air;
use airlang::syntax::escape_text;
use airlang::syntax::generate_pretty;
use airlang::syntax::parse;

use crate::cfg::BinCfg;
use crate::repl::Repl;

pub fn main() -> std::io::Result<()> {
    let mut args = args();
    if args.len() < 2 {
        let mut repl = Repl::new(stdout());
        return repl.run();
    }
    let _ = args.next().unwrap();
    let path = args.next().unwrap();
    interpret_file(&path)
}

pub fn interpret_file(path: &str) -> std::io::Result<()> {
    let source = generate_load(path);
    let mut air = Air::new(BinCfg::default().into()).unwrap();
    match parse(&source) {
        Ok(val) => {
            let output = air.interpret(val);
            match (&output).try_into() {
                Ok(o) => {
                    let mut lock = stdout().lock();
                    writeln!(lock, "{}", generate_pretty(o))
                }
                Err(e) => {
                    let mut lock = stderr().lock();
                    writeln!(lock, "{e}")
                }
            }
        }
        Err(e) => {
            let mut lock = stderr().lock();
            writeln!(lock, "{e}")
        }
    }
}

fn generate_load(path: &str) -> String {
    let mut escaped = String::new();
    escape_text(&mut escaped, path);
    format!(
        "_ do [\
            load = _ import .build.load,\
            _ load \"{escaped}\"\
        ]"
    )
}
