use std::env::args;
use std::io::IsTerminal;
use std::io::Read;
use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;

use airlang::cfg::error::ABORT_MSG;
use airlang::cfg::error::ABORT_TYPE;
use airlang::cfg::prelude;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::Eval;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::syntax::FmtOptions;
use airlang::syntax::FmtRepr;
use airlang::type_::Key;
use airlang::type_::Text;

use crate::cfg::comp::BinCompCfg;
use crate::repl::WebRepl;

pub fn main() -> std::io::Result<()> {
    if !stdin().is_terminal() || !stdout().is_terminal() {
        return cmd_run();
    }
    if args().len() >= 2 {
        return cmd_run();
    }
    WebRepl::new().run()
}

fn cmd_run() -> std::io::Result<()> {
    let source = if !stdin().is_terminal() {
        let mut input = String::new();
        stdin().read_to_string(&mut input)?;
        input
    } else {
        let mut args = args();
        if args.len() < 2 {
            return Ok(());
        }
        let _ = args.next().unwrap();
        let path = args.next().unwrap();
        generate_load(&path)
    };
    eval(&source)
}

pub fn eval(source: &str) -> std::io::Result<()> {
    let mut cfg = BinCompCfg::generate();
    let mut ctx = prelude(&mut cfg);
    match source.parse::<Val>() {
        Ok(val) => {
            let output = Eval.call(&mut cfg, Ctx::new_mut(&mut ctx), val);
            if cfg.is_aborted() { print_abort(&cfg) } else { writeln!(stdout(), "{output:#}") }
        },
        Err(e) => writeln!(stdout(), "{e}"),
    }
}

fn print_abort(cfg: &Cfg) -> std::io::Result<()> {
    let type_ = cfg.import(Key::from_str_unchecked(ABORT_TYPE));
    let msg = cfg.import(Key::from_str_unchecked(ABORT_MSG));
    match (type_, msg) {
        (Some(type_), Some(msg)) => write!(stderr(), "aborted by {type_}: {msg}"),
        (None, Some(msg)) => write!(stderr(), "aborted: {msg}"),
        (Some(type_), None) => write!(stderr(), "aborted by {type_}"),
        (None, None) => write!(stderr(), "aborted"),
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
