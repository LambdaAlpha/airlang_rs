use std::env::args;
use std::io::stderr;
use std::io::stdout;

use airlang::cfg::CoreCfg;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::type_::Text;

use crate::cfg2::BinCfg2;
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
    use std::io::Write;
    let source = generate_load(path);
    let mut cfg = BinCfg2::generate();
    let mut ctx = CoreCfg::prelude(&mut cfg, "interpret_file").unwrap();
    match source.parse::<Val>() {
        Ok(val) => {
            let output = Eval.call(&mut cfg, &mut ctx, val);
            let mut lock = stdout().lock();
            writeln!(lock, "{output:#}")
        },
        Err(e) => {
            let mut lock = stderr().lock();
            writeln!(lock, "{e}")
        },
    }
}

// AIR CODE
fn generate_load(path: &str) -> String {
    use std::fmt::Write;
    let mut escaped = String::new();
    write!(&mut escaped, "{:-}", Text::from(path)).unwrap();
    format!(
        "_ do [\
            .load set _ import _build.load,\
            _ load \"{escaped}\"\
        ]"
    )
}
