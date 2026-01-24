use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;

use airlang::cfg::CfgMod;
use airlang::cfg::error::illegal_ctx;
use airlang::cfg::error::illegal_input;
use airlang::cfg::extend_func;
use airlang::cfg::lib::DynPrimFn;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::free_impl;
use airlang::cfg::lib::mut_impl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_ID;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::MutPrimFuncVal;
use airlang::semantics::val::Val;
use const_format::concatcp;
use log::error;

// todo design
#[derive(Clone)]
pub struct IoLib {
    pub read_line: MutPrimFuncVal,
    pub print: FreePrimFuncVal,
    pub print_line: FreePrimFuncVal,
    pub flush: FreePrimFuncVal,
    pub error_print: FreePrimFuncVal,
    pub error_print_line: FreePrimFuncVal,
    pub error_flush: FreePrimFuncVal,
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
            read_line: read_line(),
            print: print(),
            print_line: print_line(),
            flush: flush(),
            error_print: error_print(),
            error_print_line: error_print_line(),
            error_flush: error_flush(),
        }
    }
}

impl CfgMod for IoLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, READ_LINE, self.read_line);
        extend_func(cfg, PRINT, self.print);
        extend_func(cfg, PRINT_LINE, self.print_line);
        extend_func(cfg, FLUSH, self.flush);
        extend_func(cfg, ERROR_PRINT, self.error_print);
        extend_func(cfg, ERROR_PRINT_LINE, self.error_print_line);
        extend_func(cfg, ERROR_FLUSH, self.error_flush);
    }
}

pub fn read_line() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_read_line) }.mut_()
}

fn fn_read_line(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Text(t) = ctx else {
        error!("ctx {ctx:?} should be a text");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let _ = stdin().read_line(t);
    Val::default()
}

pub fn print() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_print) }.free()
}

fn fn_print(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    print!("{}", &**t);
    Val::default()
}

pub fn print_line() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_print_line) }.free()
}

fn fn_print_line(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    println!("{}", &**t);
    Val::default()
}

pub fn flush() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_flush) }.free()
}

fn fn_flush(cfg: &mut Cfg, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let _ = stdout().flush();
    Val::default()
}

pub fn error_print() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_error_print) }.free()
}

fn fn_error_print(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    eprint!("{}", &**t);
    Val::default()
}

pub fn error_print_line() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_error_print_line) }.free()
}

fn fn_error_print_line(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return illegal_input(cfg);
    };
    eprintln!("{}", &**t);
    Val::default()
}

pub fn error_flush() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_error_flush) }.free()
}

fn fn_error_flush(cfg: &mut Cfg, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let _ = stderr().flush();
    Val::default()
}
