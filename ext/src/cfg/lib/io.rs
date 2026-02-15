use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;

use airlang::bug;
use airlang::cfg::CfgMod;
use airlang::cfg::extend_func;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_ID;
use airlang::semantics::func::CtxFreeInputEvalFunc;
use airlang::semantics::func::CtxFreeInputFreeFunc;
use airlang::semantics::func::CtxMutInputFreeFunc;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use const_format::concatcp;

// todo design
#[derive(Clone)]
pub struct IoLib {
    pub read_line: PrimFuncVal,
    pub print: PrimFuncVal,
    pub print_line: PrimFuncVal,
    pub flush: PrimFuncVal,
    pub error_print: PrimFuncVal,
    pub error_print_line: PrimFuncVal,
    pub error_flush: PrimFuncVal,
}

const IO: &str = "io";

pub const READ_LINE: &str = concatcp!(PREFIX_ID, IO, ".read_line");
pub const PRINT: &str = concatcp!(PREFIX_ID, IO, ".print");
pub const PRINT_LINE: &str = concatcp!(PREFIX_ID, IO, ".print_line");
pub const FLUSH: &str = concatcp!(PREFIX_ID, IO, ".flush");
pub const ERROR_PRINT: &str = concatcp!(PREFIX_ID, IO, ".error_print");
pub const ERROR_PRINT_LINE: &str = concatcp!(PREFIX_ID, IO, ".error_print_line");
pub const ERROR_FLUSH: &str = concatcp!(PREFIX_ID, IO, ".error_flush");

impl Default for IoLib {
    fn default() -> Self {
        Self {
            read_line: CtxMutInputFreeFunc { fn_: read_line }.build(),
            print: CtxFreeInputEvalFunc { fn_: print }.build(),
            print_line: CtxFreeInputEvalFunc { fn_: print_line }.build(),
            flush: CtxFreeInputFreeFunc { fn_: flush }.build(),
            error_print: CtxFreeInputEvalFunc { fn_: error_print }.build(),
            error_print_line: CtxFreeInputEvalFunc { fn_: error_print_line }.build(),
            error_flush: CtxFreeInputFreeFunc { fn_: error_flush }.build(),
        }
    }
}

impl CfgMod for IoLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, READ_LINE, self.read_line);
        extend_func(cfg, PRINT, self.print);
        extend_func(cfg, PRINT_LINE, self.print_line);
        extend_func(cfg, FLUSH, self.flush);
        extend_func(cfg, ERROR_PRINT, self.error_print);
        extend_func(cfg, ERROR_PRINT_LINE, self.error_print_line);
        extend_func(cfg, ERROR_FLUSH, self.error_flush);
    }
}

pub fn read_line(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::Text(t) = ctx else {
        return bug!(cfg, "{READ_LINE}: expected context to be a text, but got {ctx}");
    };
    let _ = stdin().read_line(t);
    Val::default()
}

pub fn print(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        return bug!(cfg, "{PRINT}: expected input to be a text, but got {input}");
    };
    print!("{}", &**t);
    Val::default()
}

pub fn print_line(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        return bug!(cfg, "{PRINT_LINE}: expected input to be a text, but got {input}");
    };
    println!("{}", &**t);
    Val::default()
}

pub fn flush(_cfg: &mut Cfg) -> Val {
    let _ = stdout().flush();
    Val::default()
}

pub fn error_print(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        return bug!(cfg, "{ERROR_PRINT}: expected input to be a text, but got {input}");
    };
    eprint!("{}", &**t);
    Val::default()
}

pub fn error_print_line(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        return bug!(cfg, "{ERROR_PRINT_LINE}: expected input to be a text, but got {input}");
    };
    eprintln!("{}", &**t);
    Val::default()
}

pub fn error_flush(_cfg: &mut Cfg) -> Val {
    let _ = stderr().flush();
    Val::default()
}
