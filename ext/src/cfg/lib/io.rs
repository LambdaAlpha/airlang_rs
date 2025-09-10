use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;

use airlang::cfg::CfgMod;
use airlang::cfg::lib::DynPrimFn;
use airlang::cfg::lib::FreePrimFn;
use airlang::cfg::lib::Library;
use airlang::cfg::lib::free_impl;
use airlang::cfg::lib::mut_impl;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::memo::Memo;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::MutPrimFuncVal;
use airlang::semantics::val::Val;
use log::error;

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
        self.read_line.extend(cfg);
        self.print.extend(cfg);
        self.print_line.extend(cfg);
        self.flush.extend(cfg);
        self.error_print.extend(cfg);
        self.error_print_line.extend(cfg);
        self.error_flush.extend(cfg);
    }
}

impl Library for IoLib {
    fn prelude(&self, _memo: &mut Memo) {}
}

pub fn read_line() -> MutPrimFuncVal {
    DynPrimFn { id: "io.read_line", f: mut_impl(fn_read_line) }.mut_()
}

fn fn_read_line(_cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Text(t) = ctx else {
        error!("ctx {ctx:?} should be a text");
        return Val::default();
    };
    let _ = stdin().read_line(t);
    Val::default()
}

pub fn print() -> FreePrimFuncVal {
    FreePrimFn { id: "io.print", f: free_impl(fn_print) }.free()
}

fn fn_print(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    print!("{}", &**t);
    Val::default()
}

pub fn print_line() -> FreePrimFuncVal {
    FreePrimFn { id: "io.print_line", f: free_impl(fn_print_line) }.free()
}

fn fn_print_line(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    println!("{}", &**t);
    Val::default()
}

pub fn flush() -> FreePrimFuncVal {
    FreePrimFn { id: "io.flush", f: free_impl(fn_flush) }.free()
}

fn fn_flush(_cfg: &mut Cfg, _input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

pub fn error_print() -> FreePrimFuncVal {
    FreePrimFn { id: "io.error_print", f: free_impl(fn_error_print) }.free()
}

fn fn_error_print(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    eprint!("{}", &**t);
    Val::default()
}

pub fn error_print_line() -> FreePrimFuncVal {
    FreePrimFn { id: "io.error_print_line", f: free_impl(fn_error_print_line) }.free()
}

fn fn_error_print_line(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Text(t) = input else {
        error!("input {input:?} should be a text");
        return Val::default();
    };
    eprintln!("{}", &**t);
    Val::default()
}

pub fn error_flush() -> FreePrimFuncVal {
    FreePrimFn { id: "io.error_flush", f: free_impl(fn_error_flush) }.free()
}

fn fn_error_flush(_cfg: &mut Cfg, _input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
