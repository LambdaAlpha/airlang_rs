use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::io::stdout;

use airlang::prelude::DynPrimFn;
use airlang::prelude::FreePrimFn;
use airlang::prelude::Prelude;
use airlang::prelude::free_impl;
use airlang::prelude::mut_impl;
use airlang::prelude::setup::default_dyn_mode;
use airlang::prelude::setup::default_free_mode;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::FreePrimFuncVal;
use airlang::semantics::val::MutPrimFuncVal;
use airlang::semantics::val::Val;
use log::error;

pub struct IoPrelude {
    pub read_line: MutPrimFuncVal,
    pub print: FreePrimFuncVal,
    pub print_line: FreePrimFuncVal,
    pub flush: FreePrimFuncVal,
    pub error_print: FreePrimFuncVal,
    pub error_print_line: FreePrimFuncVal,
    pub error_flush: FreePrimFuncVal,
}

impl Default for IoPrelude {
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

impl Prelude for IoPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.read_line.put(ctx);
        self.print.put(ctx);
        self.print_line.put(ctx);
        self.flush.put(ctx);
        self.error_print.put(ctx);
        self.error_print_line.put(ctx);
        self.error_flush.put(ctx);
    }
}

pub fn read_line() -> MutPrimFuncVal {
    DynPrimFn { id: "io.read_line", f: mut_impl(fn_read_line), mode: default_dyn_mode() }.mut_()
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
    FreePrimFn { id: "io.print", f: free_impl(fn_print), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "io.print_line", f: free_impl(fn_print_line), mode: default_free_mode() }
        .free()
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
    FreePrimFn { id: "io.flush", f: free_impl(fn_flush), mode: default_free_mode() }.free()
}

fn fn_flush(_cfg: &mut Cfg, _input: Val) -> Val {
    let _ = stdout().flush();
    Val::default()
}

pub fn error_print() -> FreePrimFuncVal {
    FreePrimFn { id: "io.error_print", f: free_impl(fn_error_print), mode: default_free_mode() }
        .free()
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
    FreePrimFn {
        id: "io.error_print_line",
        f: free_impl(fn_error_print_line),
        mode: default_free_mode(),
    }
    .free()
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
    FreePrimFn { id: "io.error_flush", f: free_impl(fn_error_flush), mode: default_free_mode() }
        .free()
}

fn fn_error_flush(_cfg: &mut Cfg, _input: Val) -> Val {
    let _ = stderr().flush();
    Val::default()
}
