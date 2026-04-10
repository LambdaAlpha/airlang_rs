use std::env::args;
use std::io::stderr;
use std::io::stdout;

use airlang::cfg::prelude;
use airlang::semantics::core::Eval;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::syntax::FmtOptions;
use airlang::syntax::FmtRepr;
use airlang::type_::Text;

use crate::cfg::comp::BinCompCfg;
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
    let mut cfg = BinCompCfg::generate();
    let mut ctx = prelude(&mut cfg);
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
    let mut path_text = String::new();
    Text::from(path).fmt(FmtOptions::default(), &mut path_text).unwrap();
    format!(
        "_ do _[\
            _load set _ import .build.load,\
            _ load {path_text}\
        ]"
    )
}
